use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use jsonwebtoken::DecodingKey;

use super::auth_types::{
    credential::{ClearCredential, Credential},
    email::EmailAddress,
    key_identifier::Kid,
    provider::AuthProvider,
};
use crate::Result;

pub struct Token(pub String);

#[async_trait]
pub trait IAuthService {
    async fn login_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<Token>;
    async fn login_credential(&self, credential: &ClearCredential) -> Result<Token>;
    async fn register_credential(&self, credential: &Credential) -> Result<Token>;
    async fn register_provider(
        &self,
        provider_token: &Token,
        provider: &AuthProvider,
        key_set: &Mutex<ProviderKeySet>,
    ) -> Result<Token>;
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
