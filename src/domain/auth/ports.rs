use std::collections::HashMap;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};

use super::{
    jwt_authentication::Claims,
    provider::AuthProvider,
    value_object::{email::EmailAddress, key_identifier::Kid, password::Password},
};
use crate::Result;

pub struct Token(pub String);

pub struct Credential {
    email: EmailAddress,
    password: Option<Password>,
}

#[async_trait]
pub trait IAuthService {
    async fn provider_login(&self, provider_token: &Token, provider: AuthProvider)
        -> Result<Token>;
    async fn register_credential(&self, credential: &Credential) -> Result<Token>;
    async fn register_provider(&self, provider_token: &Token) -> Result<Token>;
}

pub struct ProviderKeySet {
    pub keys: HashMap<String, DecodingKey<'static>>,
    pub expiration: NaiveDateTime,
}

#[async_trait]
pub trait Repository {
    async fn get_key_set(&self) -> Result<ProviderKeySet>;
    async fn update_key_set(&self, provider_key_set: &mut ProviderKeySet) -> Result<()>;
    async fn check_existing_user(&self, provider_subject: &str) -> Result<i32>;
    async fn create_user(&self, credential: &Credential) -> Result<i32>;
    async fn user_id(&self, token: &Token) -> Result<i32>;
}

pub trait HandleJWT {
    fn generate_token(&self, claims: &Claims, encoding_key: &EncodingKey) -> Result<Token> {
        Ok(Token(encode(&Header::default(), claims, encoding_key)?))
    }
}
