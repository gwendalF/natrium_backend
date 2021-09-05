use super::{
    email::{EmailAddress, EmailError},
    password::{Password, PasswordError},
    salt::Salt,
};
use thiserror::Error;

pub struct Credential {
    pub email: EmailAddress,
    hash: Password,
}

#[derive(Error, Debug)]
pub enum CredentialError {
    #[error("Email error")]
    InvalidEmail(#[from] EmailError),
    #[error("Password error")]
    InvalidPassword(#[from] PasswordError),
}

pub struct CredentialDTO {
    pub email: String,
    pub password: String,
}
