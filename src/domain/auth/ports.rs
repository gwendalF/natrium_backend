use std::collections::HashMap;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};

use super::{
    auth_types::{
        credential::Credential, email::EmailAddress, key_identifier::Kid, password::Password,
        provider::AuthProvider, salt::Salt,
    },
    jwt_authentication::Claims,
};
use crate::Result;

pub struct Token(pub String);

#[async_trait]
pub trait IAuthService {
    async fn provider_login(&self, provider_token: &Token, provider: AuthProvider)
        -> Result<Token>;
    async fn credential_login(&self, credential: &Credential) -> Result<Token>;
    async fn register_credential(&self, credential: &Credential) -> Result<Token>;
    async fn register_provider(
        &self,
        provider_token: &Token,
        provider: AuthProvider,
    ) -> Result<Token>;
}

pub struct ProviderKeySet {
    pub keys: HashMap<String, DecodingKey<'static>>,
    pub expiration: NaiveDateTime,
}

#[async_trait]
pub trait UserRepository {
    async fn update_key_set(&self, provider_key_set: &mut ProviderKeySet) -> Result<()>;
    async fn check_existing_user(&self, provider_subject: &str) -> Result<i32>;
    async fn save_user_credential(&self, credential: &Credential, salt: &Salt) -> Result<i32>;
    async fn save_user_provider(&self, provider: AuthProvider, )
    async fn create_user_subject(&self, provider_subject: &str) -> Result<i32>;
    async fn credential_login(&self, credential: &Credential) -> Result<Token>;
    async fn save_credential(&self, credential: &Credential, salt: &Salt) -> Result<()>;
}
