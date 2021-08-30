use thiserror::Error;

use crate::domain::core::value_object::ValueObject;

#[derive(Debug, Clone)]
pub struct Kid(Result<String, KidError>);

impl Kid {
    pub fn new(value: String) -> Kid {
        Kid(validate(value))
    }
}

fn validate(value: String) -> Result<String, KidError> {
    if value.len() > 45 {
        Err(KidError::InvalidKid)
    } else {
        Ok(value)
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum KidError {
    #[error("Invalid kid")]
    InvalidKid,
}

impl ValueObject<String, KidError> for Kid {
    fn value(&self) -> Result<&String, &KidError> {
        self.0.as_ref()
    }

    fn is_valid(&self) -> bool {
        self.0.is_ok()
    }
}
