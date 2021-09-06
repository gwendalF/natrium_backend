use super::{
    email::{EmailAddress, EmailError},
    password::{Password, PasswordError},
    salt::Salt,
};
use thiserror::Error;

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

pub struct ClearCredential {
    email: EmailAddress,
    password: String,
}
