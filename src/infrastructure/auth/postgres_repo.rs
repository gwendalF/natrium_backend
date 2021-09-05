use std::collections::HashMap;

use crate::domain::auth::auth_types::credential::Credential;
use crate::domain::auth::auth_types::salt::Salt;
use crate::domain::auth::ports::{ProviderKeySet, Token, UserRepository};

use crate::domain::auth::auth_types::email::EmailAddress;
use crate::{AppError, Result};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
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
    async fn update_key_set(&self, provider_key_set: &mut ProviderKeySet) -> Result<()> {
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
            .ok_or_else(|| AppError::ServerError)?;
        let expiration = capture
            .get(1)
            .ok_or_else(|| AppError::ServerError)?
            .as_str();
        let expiration = NaiveDateTime::from_timestamp(expiration.parse::<i64>()?, 0);
        let new_key_set = response
            .json::<HashMap<String, Vec<HashMap<String, String>>>>()
            .await?;
        let mut new_keys = HashMap::with_capacity(new_key_set["keys"].len());
        for decoding_key in &new_key_set["keys"] {
            sqlx::query!(
                r#"
                INSERT INTO token_key(kid, provider, modulus, exponent, expiration)
                VALUES ($1, 'google', $2, $3, $4)
                ON CONFLICT (kid) DO UPDATE 
                SET
                    kid = $1,
                    modulus = $2,
                    exponent = $3,
                    expiration = $4
                    WHERE token_key.kid = $1
            "#,
                &decoding_key["kid"],
                &decoding_key["n"],
                &decoding_key["e"],
                &expiration
            )
            .execute(&self.repo)
            .await?;
            let key = DecodingKey::from_rsa_components(&decoding_key["n"], &decoding_key["e"])
                .into_static();
            new_keys.insert(decoding_key["kid"].to_owned(), key);
        }
        let provider_key = ProviderKeySet {
            keys: new_keys,
            expiration,
        };
        *provider_key_set = provider_key;
        Ok(())
    }

    async fn check_existing_user(&self, provider_subject: &str) -> Result<i32> {
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
            Err(AppError::UserNotFoundEror)
        }
    }

    async fn create_user_credential(&self, credential: &Credential) -> Result<i32> {
        todo!()
    }

    async fn save_credential(&self, credential: &Credential, salt: &Salt) -> Result<()> {
        todo!()
    }

    async fn create_user_subject(&self, provider_subject: &str) -> Result<i32> {
        todo!()
    }

    async fn credential_login(&self, credential: &Credential) -> Result<Token> {
        todo!()
    }
}
