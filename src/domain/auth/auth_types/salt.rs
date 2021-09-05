use rand::Rng;
use thiserror::Error;
pub struct Salt(String);

#[derive(Error, Debug)]
pub enum SaltError {
    #[error("Invalid salt")]
    InvalidSalt,
}

fn validate(value: String) -> Result<String, SaltError> {
    if value.len() != 32 {
        Err(SaltError::InvalidSalt)
    } else {
        Ok(value)
    }
}

impl Salt {
    pub fn new() -> Salt {
        let salt: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        Salt(salt)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
