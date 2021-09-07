use super::{
    email::{EmailAddress, EmailError},
    password::{Password, PasswordError},
    salt::Salt,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
pub struct Credential {
    pub email: EmailAddress,
    pub hash: Password,
    pub salt: Salt,
}

#[derive(Error, Debug)]
pub enum CredentialError {
    #[error("Email error")]
    InvalidEmail(#[from] EmailError),
    #[error("Password error")]
    InvalidPassword(#[from] PasswordError),
}

#[derive(Deserialize)]
pub struct ClearCredential {
    pub email: String,
    pub password: String,
}
