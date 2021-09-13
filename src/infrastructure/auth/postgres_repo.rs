use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::auth::auth_types::credential::Credential;
use crate::domain::auth::auth_types::key_identifier::Kid;

use crate::domain::auth::auth_types::password::PasswordError;
use crate::domain::auth::errors::AuthError;
use crate::domain::auth::ports::{ProviderKeySet, UserRepository};

use crate::domain::auth::auth_types::email::{EmailAddress, EmailError};
use crate::{AppError, Result};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::DecodingKey;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::CACHE_CONTROL;
use sqlx::PgPool;

pub struct UserRepositoryImpl {
    pub repo: PgPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn update_key_set(&self, provider_key_set: &Mutex<ProviderKeySet>) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new("max-age=([0-9]+)").unwrap();
        }
        let response = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await?;
        let header = response.headers()[CACHE_CONTROL].clone();
        let capture = RE
            .captures(
                header
                    .to_str()
                    .expect("Google key header cache-control cannot be used as str"),
            )
            .ok_or(AppError::ServerError)?;
        let expiration = capture.get(1).ok_or(AppError::ServerError)?.as_str();
        let expiration = (Utc::now() + Duration::seconds(expiration.parse::<i64>()?)).naive_utc();
        let new_key_set = response
            .json::<HashMap<String, Vec<HashMap<String, String>>>>()
            .await?;
        let mut new_keys = HashMap::with_capacity(new_key_set["keys"].len());
        let mut transaction = self.repo.begin().await?;
        for decoding_key in &new_key_set["keys"] {
            sqlx::query!(
                r#"
                INSERT INTO token_key(kid, provider, modulus, exponent, expiration, created_at, updated_at)
                VALUES ($1, 'google', $2, $3, $4, $5, $5)
                ON CONFLICT (kid) DO UPDATE 
                SET
                    kid = $1,
                    modulus = $2,
                    exponent = $3,
                    expiration = $4,
                    updated_at = $5
                    WHERE token_key.kid = $1
            "#,
                &decoding_key["kid"],
                &decoding_key["n"],
                &decoding_key["e"],
                &expiration,
                chrono::Utc::now().naive_utc()
            )
            .execute(&mut transaction)
            .await?;
            let key = DecodingKey::from_rsa_components(&decoding_key["n"], &decoding_key["e"])
                .into_static();
            let kid =
                Kid::new(decoding_key["kid"].to_owned()).expect("Manage different error type");
            new_keys.insert(kid, key);
        }
        sqlx::query!(
            r#"
            DELETE FROM token_key WHERE expiration < NOW()
        "#
        )
        .execute(&mut transaction)
        .await?;

        let provider_key = ProviderKeySet {
            keys: new_keys,
            expiration,
        };
        {
            let mut key_set = provider_key_set.lock().expect("Lock error");
            *key_set = provider_key;
        }
        transaction.commit().await?;
        Ok(())
    }

    async fn check_existing_user_subject(&self, provider_subject: &str) -> Result<i32> {
        if let Some(record) = sqlx::query!(
            r#"
        SELECT user_account.id from user_account JOIN provider_user_mapper
        ON (user_account.id = provider_user_mapper.user_id)
        WHERE provider_user_mapper.subject = $1
        "#,
            provider_subject
        )
        .fetch_optional(&self.repo)
        .await?
        {
            Ok(record.id)
        } else {
            Err(AuthError::Email(EmailError::InvalidEmail).into())
        }
    }

    async fn create_user_credential(&self, credential: &Credential) -> Result<i32> {
        if let Some(user_id) = sqlx::query!(
            r#"
            INSERT INTO user_account (email, password, salt, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            ON CONFLICT DO NOTHING
            RETURNING id
        "#,
            &credential.email.value(),
            &credential.hash.value(),
            &credential.salt.value(),
            chrono::Utc::now().naive_utc()
        )
        .fetch_optional(&self.repo)
        .await?
        {
            Ok(user_id.id)
        } else {
            Err(AuthError::Email(EmailError::AlreadyUsedEmail).into())
        }
    }

    async fn create_user_subject(
        &self,
        provider_subject: &str,
        provider_email: &EmailAddress,
    ) -> Result<i32> {
        let mut transaction = self.repo.begin().await?;
        let user_id = sqlx::query!(
            r#"
            INSERT INTO user_account (email, created_at, updated_at)
            VALUES ($1, $2, $2)
            ON CONFLICT DO NOTHING
            RETURNING id
        "#,
            &provider_email.value(),
            chrono::Utc::now().naive_utc()
        )
        .fetch_one(&mut transaction)
        .await?
        .id;
        sqlx::query!(
            r#"
            INSERT INTO provider_user_mapper (name, subject, user_id)
            VALUES ('google', $1, $2)           
        "#,
            provider_subject,
            user_id
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(user_id)
    }

    async fn check_existing_user_email(&self, email: &EmailAddress) -> Result<i32> {
        Ok(sqlx::query!(
            r#"
            SELECT id FROM user_account 
            WHERE email=$1
        "#,
            email.value()
        )
        .fetch_one(&self.repo)
        .await?
        .id)
    }

    async fn hash(&self, email: &EmailAddress) -> Result<String> {
        if let Some(user_hash_record) = sqlx::query!(
            r#"
            SELECT password FROM user_account 
            WHERE email=$1
        "#,
            email.value()
        )
        .fetch_optional(&self.repo)
        .await?
        {
            if let Some(hash) = user_hash_record.password {
                Ok(hash)
            } else {
                Err(AuthError::Password(PasswordError::PasswordNotFound).into())
            }
        } else {
            Err(AuthError::Password(PasswordError::PasswordNotFound).into())
        }
    }
}
