use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, encode, Header, Validation};

use crate::domain::auth::{
    auth_types::{
        credential::{ClearCredential, Credential, CredentialError},
        email::EmailAddress,
        key_identifier::{Kid, KidError},
        password::{Password, PasswordError},
        provider::AuthProvider,
    },
    errors::AuthError,
    jwt_authentication::{AppKey, Claims, ProviderClaims},
    ports::{IAuthService, Token, UserRepository},
};
use crate::Result;
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
        provider: &AuthProvider,
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

    async fn login_credential(&self, credential: &ClearCredential) -> Result<Token> {
        let user_email =
            EmailAddress::new(credential.email.clone()).expect("error management really needed");
        let user_id = self
            .repository
            .check_existing_user_email(&user_email)
            .await?;
        let user_hash = self.repository.hash(&user_email).await?;
        let password_valid = argon2::verify_encoded(&user_hash, &credential.password.as_bytes())?;
        if password_valid {
            let claims = Claims::new(user_id);
            Ok(Token(encode(
                &Header::default(),
                &claims,
                &self.application_key.encoding,
            )?))
        } else {
            Err(AuthError::Password(PasswordError::InvalidPassword))?
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
        provider: &AuthProvider,
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
    provider: &AuthProvider,
    token: &Token,
    repository: &T,
) -> Result<ProviderClaims>
where
    T: UserRepository + Sync + Send,
{
    let header = decode_header(&token.0)?;
    match provider {
        AuthProvider::Facebook => unimplemented!(),
        AuthProvider::Google(key_set) => {
            let mut key_set = key_set.lock().expect("Mutex lock error");
            if key_set.expiration <= chrono::Utc::now().naive_utc() {
                repository.update_key_set(&mut key_set).await?;
            }
            let kid = Kid::new(
                header
                    .kid
                    .ok_or_else(|| AuthError::Kid(KidError::InvalidKid))?,
            )
            .map_err(|_| AuthError::Kid(KidError::InvalidKid))?;
            let decoding_key = &key_set.keys[&kid];
            let provider_claims =
                decode::<ProviderClaims>(&token.0, decoding_key, &Validation::default())?.claims;
            match provider_claims.iss.as_ref() {
                "accounts.google.com" | "https://accounts.google.com" => Ok(provider_claims),
                _ => Err(AuthError::Token)?,
            }
        }
    }
}
