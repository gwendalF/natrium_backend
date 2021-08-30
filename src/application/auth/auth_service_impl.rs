use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, encode, Validation};

use crate::domain::{
    auth::{
        jwt_authentication::{AppKey, Claims},
        ports::{Credential, IAuthService, Repository, Token},
        provider::AuthProvider,
        value_object::key_identifier::Kid,
    },
    core::value_object::ValueObject,
};
use crate::{AppError, Result};
pub struct AuthService<T> {
    pub repository: T,
    pub application_key: AppKey,
}

#[async_trait]
impl<T> IAuthService for AuthService<T>
where
    T: Repository + Send + Sync,
{
    async fn provider_login(
        &self,
        provider_token: &Token,
        provider: AuthProvider,
    ) -> Result<Token> {
        match provider {
            AuthProvider::Google(mut key_set) => {
                if key_set.expiration <= chrono::Utc::now().naive_utc() {
                    self.repository.update_key_set(&mut key_set).await?;
                }
                let header = decode_header(&provider_token.0)?;
                let kid =
                    Kid::new(header.kid.ok_or_else(|| {
                        AppError::TokenError("Missing key identifier".to_owned())
                    })?);
                let key = kid.value().unwrap();
                let decoding_key = &key_set.keys[key];
                let provider_claims =
                    decode::<Claims>(&provider_token.0, decoding_key, &Validation::default())?
                        .claims;
                match provider_claims.iss.as_ref() {
                    "accounts.google.com" | "https://accounts.google.com" => (),
                    _ => {
                        return Err(AppError::PermissionDenied(
                            "Malicious user detected".to_owned(),
                        ))
                    }
                }
                if let Ok(_) = self
                    .repository
                    .check_existing_user(&provider_claims.sub)
                    .await
                {
                    Ok(Token(decode()))
                }
            }
            AuthProvider::Facebook => unimplemented!(),
        }
        todo!()
    }

    async fn register_credential(&self, credential: &Credential) -> Result<Token> {
        todo!()
    }

    async fn register_provider(&self, provider_token: &Token) -> Result<Token> {
        todo!()
    }
}
