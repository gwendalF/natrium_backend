use argon2::Config;
use thiserror::Error;

use super::salt::Salt;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Password not found")]
    PasswordNotFound,
}
pub struct Password(String);

fn validate(password: String) -> Result<String, PasswordError> {
    if password.len() < 8 {
        Err(PasswordError::InvalidPassword)
    } else {
        Ok(password)
    }
}

fn validate_hash(hash: String) -> Result<String, PasswordError> {
    if hash.len() == 116 {
        Ok(hash)
    } else {
        Err(PasswordError::InvalidHash)
    }
}

impl Password {
    pub fn new(password: String, salt: Salt) -> Result<Password, PasswordError> {
        match validate(password) {
            Ok(pwd) => {
                let hash = argon2::hash_encoded(
                    pwd.as_bytes(),
                    salt.value().as_bytes(),
                    &Config::default(),
                )
                .unwrap();
                Ok(Password(hash))
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_hash(hash: String) -> Password {
        todo!()
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
