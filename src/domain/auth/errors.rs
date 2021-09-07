use thiserror::Error;

use super::auth_types::{
    credential::CredentialError, email::EmailError, key_identifier::KidError,
    password::PasswordError, salt::SaltError,
};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Credential error")]
    Credential(#[from] CredentialError),
    #[error("Email error")]
    Email(#[from] EmailError),
    #[error("Kid error")]
    Kid(#[from] KidError),
    #[error("Password error")]
    Password(#[from] PasswordError),
    #[error("Salt error")]
    Salt(#[from] SaltError),
    #[error("Token error")]
    Token,
}
