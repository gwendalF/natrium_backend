use crate::domain::core::value_object::ValueObject;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Invalid password")]
    InvalidPassword,
}
pub struct Password(Result<String, PasswordError>);

impl ValueObject<String, PasswordError> for Password {
    fn value(&self) -> Result<&String, &PasswordError> {
        self.0.as_ref()
    }

    fn is_valid(&self) -> bool {
        self.0.is_ok()
    }
}

fn validate(password: String) -> Result<String, PasswordError> {
    if password.len() < 8 {
        Err(PasswordError::InvalidPassword)
    } else {
        Ok(password)
    }
}

impl Password {
    pub fn new(password: String) -> Password {
        Password(validate(password))
    }
}
