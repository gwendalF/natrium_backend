use thiserror::Error;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Kid(String);

impl Kid {
    pub fn new(value: String) -> Result<Kid, KidError> {
        Ok(Kid(validate(value)?))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

fn validate(value: String) -> Result<String, KidError> {
    if value.len() > 45 {
        Err(KidError::InvalidKid)
    } else {
        Ok(value)
    }
}

#[derive(Error, Debug)]
pub enum KidError {
    #[error("Invalid kid")]
    InvalidKid,
}
