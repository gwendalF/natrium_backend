use std::sync::Mutex;

use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, Validation};

use crate::domain::auth::{
    auth_types::{
        credential::{ClearCredential, Credential},
        email::EmailAddress,
        key_identifier::{Kid, KidError},
        password::PasswordError,
        provider::AuthProvider,
    },
    errors::AuthError,
    jwt_authentication::{AppKey, Claims, ProviderClaims, RefreshKey},
    ports::{AuthToken, IAuthService, ProviderKeySet, Token, UserRepository},
};
use crate::Result;

#[derive(Clone)]
pub struct AuthService<T> {
    pub repository: T,
    pub application_key: AppKey,
    pub refresh_key: RefreshKey,
}

#[async_trait]
impl<T> IAuthService for AuthService<T>
where
    T: UserRepository + Send + Sync,
{
    async fn login_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<AuthToken> {
        let provider_claims =
            decode_provider(provider, provider_token, &self.repository, key_set).await?;
        let user_id = self
            .repository
            .check_existing_user_subject(&provider_claims.sub)
            .await?;
        let token = AuthToken::new(
            user_id,
            &self.application_key.encoding,
            &self.refresh_key.encoding,
        )?;
        Ok(token)
    }

    async fn login_credential(&self, credential: &ClearCredential) -> Result<AuthToken> {
        let user_email =
            EmailAddress::new(credential.email.clone()).expect("error management really needed");
        let user_id = self
            .repository
            .check_existing_user_email(&user_email)
            .await?;
        let user_hash = self.repository.hash(&user_email).await?;
        let password_valid = argon2::verify_encoded(&user_hash, &credential.password.as_bytes())?;
        if password_valid {
            let token = AuthToken::new(
                user_id,
                &self.application_key.encoding,
                &self.refresh_key.encoding,
            )?;
            Ok(token)
        } else {
            Err(AuthError::Password(PasswordError::InvalidPassword).into())
        }
    }

    async fn register_credential(&self, credential: &Credential) -> Result<AuthToken> {
        let user_id = self.repository.create_user_credential(credential).await?;
        let token = AuthToken::new(
            user_id,
            &self.application_key.encoding,
            &self.refresh_key.encoding,
        )?;
        Ok(token)
    }

    async fn register_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<AuthToken> {
        let claims = decode_provider(provider, provider_token, &self.repository, key_set).await?;
        let user_id = self
            .repository
            .create_user_subject(
                &claims.sub,
                &EmailAddress::new(claims.email).expect("Error to be managed"),
            )
            .await?;
        let token = AuthToken::new(
            user_id,
            &self.application_key.encoding,
            &self.refresh_key.encoding,
        )?;
        Ok(token)
    }

    async fn refresh_token(&self, refresh_token: &Token) -> Result<AuthToken> {
        let mut validation = Validation {
            iss: Some("natrium".to_owned()),
            ..Validation::default()
        };
        validation.set_audience(&["natrium"]);
        let refresh_claims =
            decode::<Claims>(&refresh_token.0, &self.refresh_key.decoding, &validation)?.claims;
        let user_id = self
            .repository
            .check_existing_user_subject(&refresh_claims.sub)
            .await?;
        if refresh_claims.permissions == Some(vec![format!("ACCESS_TOKEN_{}", user_id)]) {
            Ok(AuthToken::new(
                user_id,
                &self.application_key.encoding,
                &self.refresh_key.encoding,
            )?)
        } else {
            Err(AuthError::Token.into())
        }
    }
}

async fn decode_provider<T>(
    provider: &AuthProvider,
    token: &Token,
    repository: &T,
    key_set: &Mutex<ProviderKeySet>,
) -> Result<ProviderClaims>
where
    T: UserRepository + Sync + Send,
{
    let header = decode_header(&token.0)?;
    match provider {
        AuthProvider::Facebook => unimplemented!(),
        AuthProvider::Google => {
            let expiration;
            {
                let key_set_guard = key_set.lock().expect("Lock error");
                expiration = key_set_guard.expiration;
            }

            if expiration <= chrono::Utc::now().naive_utc() {
                repository.update_key_set(key_set).await?;
            }
            let kid = Kid::new(header.kid.ok_or(AuthError::Kid(KidError::InvalidKid))?)
                .map_err(|_| AuthError::Kid(KidError::InvalidKid))?;
            let key_set = key_set.lock().expect("Lock error");
            let decoding_key = &key_set.keys[&kid];
            let validation = Validation {
                algorithms: vec![jsonwebtoken::Algorithm::RS256],
                ..Validation::default()
            };
            let provider_claims =
                decode::<ProviderClaims>(&token.0, decoding_key, &validation)?.claims;
            match provider_claims.iss.as_ref() {
                "accounts.google.com" | "https://accounts.google.com" => Ok(provider_claims),
                _ => Err(AuthError::Token.into()),
            }
        }
    }
}
