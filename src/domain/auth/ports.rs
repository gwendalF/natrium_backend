use async_trait::async_trait;
use std::sync::Mutex;

use super::auth_types::{
    credential::{ClearCredential, Credential},
    email::EmailAddress,
    provider::{AuthProvider, ProviderKeySet},
    token::{AuthToken, Token},
};
use crate::Result;

#[cfg_attr(test, mockall::automock)]
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

#[cfg_attr(test, mockall::automock)]
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

#[cfg_attr(test, mockall::automock)]
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
