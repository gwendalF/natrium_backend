use std::convert::TryFrom;

use crate::domain::auth::errors::AuthError;

use super::{
    email::{EmailAddress, EmailError},
    password::{Password, PasswordError},
    salt::Salt,
};

use serde::Deserialize;
use thiserror::Error;

pub struct Credential {
    pub email: EmailAddress,
    pub hash: Password,
    pub salt: Salt,
}

impl TryFrom<ClearCredential> for Credential {
    type Error = AuthError;
    fn try_from(value: ClearCredential) -> Result<Self, Self::Error> {
        let salt = Salt::new();
        let hash = Password::new(value.password, &salt)?;
        let email = EmailAddress::new(value.email)?;
        Ok(Credential { salt, hash, email })
    }
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
