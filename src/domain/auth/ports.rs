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
    async fn login_provider(&self, provider_token: &Token, provider: AuthProvider)
        -> Result<Token>;
    async fn login_credential(&self, credential: &Credential) -> Result<Token>;
    async fn register_credential(&self, credential: &Credential) -> Result<Token>;
    async fn register_provider(
        &self,
        provider_token: &Token,
        provider: AuthProvider,
    ) -> Result<Token>;
}

pub struct ProviderKeySet {
    pub keys: HashMap<Kid, DecodingKey<'static>>,
    pub expiration: NaiveDateTime,
}

#[async_trait]
pub trait UserRepository {
    async fn update_key_set(&self, provider_key_set: &mut ProviderKeySet) -> Result<()>;
    async fn check_existing_user_provider(&self, provider_subject: &str) -> Result<i32>;
    async fn check_existing_user_email(&self, email: &EmailAddress) -> Result<i32>;
    async fn validate_password(&self, credential: &Credential) -> bool;
    async fn create_user_subject(
        &self,
        provider_subject: &str,
        provider_email: &EmailAddress,
    ) -> Result<i32>;
    async fn create_user_credential(&self, credential: &Credential) -> Result<i32>;
}
