use std::{collections::HashMap, sync::Mutex};

use actix_web::{cookie::Cookie, HttpResponse};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};

use super::{
    auth_types::{
        credential::{ClearCredential, Credential},
        email::EmailAddress,
        key_identifier::Kid,
        provider::AuthProvider,
    },
    jwt_authentication::{Claims, TokenType, REFRESH_TOKEN_DURATION},
};
use crate::Result;

#[derive(Debug, serde::Serialize)]
pub struct Token(pub String);

#[derive(Debug, serde::Serialize)]
pub struct AuthToken {
    pub refresh_token: Token,
    pub access_token: Token,
    pub expiration: usize,
}

impl AuthToken {
    pub fn new(
        user_id: i32,
        access_key: &EncodingKey,
        refresh_key: &EncodingKey,
    ) -> Result<AuthToken> {
        let access_claims = Claims::new(user_id, TokenType::AccessToken);
        let refresh_claims = Claims::new(user_id, TokenType::RefreshToken);
        let expiration = access_claims.exp;
        let access_token = Token(encode(&Header::default(), &access_claims, access_key)?);
        let refresh_token = Token(encode(&Header::default(), &refresh_claims, refresh_key)?);
        Ok(AuthToken {
            refresh_token,
            access_token,
            expiration,
        })
    }
}

impl From<AuthToken> for HttpResponse {
    fn from(token: AuthToken) -> Self {
        let when = time::OffsetDateTime::now_utc() + time::Duration::days(REFRESH_TOKEN_DURATION);
        let cookie = Cookie::build("refresh_token", &token.refresh_token.0)
            .path("/refresh_token/")
            .secure(true)
            .expires(when)
            .http_only(true)
            .finish();
        #[derive(serde::Serialize)]
        struct AccessToken {
            access_token: String,
            expiration: usize,
        }
        HttpResponse::build(actix_web::http::StatusCode::OK)
            .cookie(cookie)
            .json(AccessToken {
                access_token: token.access_token.0,
                expiration: token.expiration,
            })
    }
}

#[async_trait]
pub trait IAuthService {
    async fn login_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<AuthToken>;
    async fn login_credential(&self, credential: &ClearCredential) -> Result<AuthToken>;
    async fn register_credential(&self, credential: &Credential) -> Result<AuthToken>;
    async fn register_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<AuthToken>;
    async fn refresh_token(&self, refresh_token: &Token) -> Result<AuthToken>;
}

#[derive(Debug, Clone)]
pub struct ProviderKeySet {
    pub keys: HashMap<Kid, DecodingKey<'static>>,
    pub expiration: NaiveDateTime,
}

#[async_trait]
pub trait UserRepository {
    async fn update_key_set(&self, provider_key_set: &Mutex<ProviderKeySet>) -> Result<()>;
    async fn check_existing_user_provider(&self, provider_subject: &str) -> Result<i32>;
    async fn check_existing_user_email(&self, email: &EmailAddress) -> Result<i32>;
    async fn hash(&self, email: &EmailAddress) -> Result<String>;
    async fn create_user_subject(
        &self,
        provider_subject: &str,
        provider_email: &EmailAddress,
    ) -> Result<i32>;
    async fn create_user_credential(&self, credential: &Credential) -> Result<i32>;
}

#[async_trait]
pub trait TokenRepository {
    async fn save_token(
        &self,
        user_id: i32,
        refresh_token: &Token,
        expiration: usize,
    ) -> Result<()>;
    async fn check_existing_token(&self, user_id: i32, refresh_token: &Token) -> Result<()>;
}
