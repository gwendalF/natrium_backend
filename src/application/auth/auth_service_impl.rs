use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, encode, Header, Validation};

use crate::domain::auth::{
    auth_types::{
        credential::{self, Credential},
        email::EmailAddress,
        key_identifier::Kid,
        provider::AuthProvider,
    },
    jwt_authentication::{AppKey, Claims, ProviderClaims},
    ports::{IAuthService, ProviderKeySet, Token, UserRepository},
};
use crate::{AppError, Result};
pub struct AuthService<T> {
    pub repository: T,
    pub application_key: AppKey,
}

#[async_trait]
impl<T> IAuthService for AuthService<T>
where
    T: UserRepository + Send + Sync,
{
    async fn login_provider(
        &self,
        provider_token: &Token,
        provider: AuthProvider,
    ) -> Result<Token> {
        let provider_claims = decode_provider(provider, provider_token, &self.repository).await?;
        let user_id = self
            .repository
            .check_existing_user_provider(&provider_claims.sub)
            .await?;
        let claims = Claims::new(user_id);
        Ok(Token(encode(
            &Header::default(),
            &claims,
            &self.application_key.encoding,
        )?))
    }
    async fn login_credential(&self, credential: &Credential) -> Result<Token> {
        let user_id = self
            .repository
            .check_existing_user_email(&credential.email)
            .await?;
        if self.repository.validate_password(&credential).await {
            let claims = Claims::new(user_id);
            Ok(Token(encode(
                &Header::default(),
                &claims,
                &self.application_key.encoding,
            )?))
        } else {
            Err(AppError::PermissionDenied(
                "Wrong email/password combination".to_owned(),
            ))
        }
    }

    async fn register_credential(&self, credential: &Credential) -> Result<Token> {
        let user_id = self.repository.create_user_credential(credential).await?;
        let claims = Claims::new(user_id);
        let token = Token(encode(
            &Header::default(),
            &claims,
            &self.application_key.encoding,
        )?);
        Ok(token)
    }

    async fn register_provider(
        &self,
        provider_token: &Token,
        provider: AuthProvider,
    ) -> Result<Token> {
        let claims = decode_provider(provider, provider_token, &self.repository).await?;
        let user_id = self
            .repository
            .create_user_subject(
                &claims.sub,
                &EmailAddress::new(claims.email).expect("Error to be managed"),
            )
            .await?;
        let claims = Claims::new(user_id);
        Ok(Token(encode(
            &Header::default(),
            &claims,
            &self.application_key.encoding,
        )?))
    }
}

async fn decode_provider<T>(
    provider: AuthProvider,
    token: &Token,
    repository: &T,
) -> Result<ProviderClaims>
where
    T: UserRepository + Sync + Send,
{
    let header = decode_header(&token.0)?;
    match provider {
        AuthProvider::Facebook => unimplemented!(),
        AuthProvider::Google(mut key_set) => {
            if key_set.expiration <= chrono::Utc::now().naive_utc() {
                repository.update_key_set(&mut key_set).await?;
            }
            let kid = Kid::new(
                header
                    .kid
                    .ok_or_else(|| AppError::TokenError("Missing key identifier".to_owned()))?,
            )
            .map_err(|_| AppError::TokenError("Key identifier too long".to_owned()))?;
            let decoding_key = &key_set.keys[&kid];
            let provider_claims =
                decode::<ProviderClaims>(&token.0, decoding_key, &Validation::default())?.claims;
            match provider_claims.iss.as_ref() {
                "accounts.google.com" | "https://accounts.google.com" => Ok(provider_claims),
                _ => Err(AppError::PermissionDenied(
                    "Malicious user detected".to_owned(),
                )),
            }
        }
    }
}
