use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("JWT is invalid")]
    WrongToken,
    #[error("Wrong email/password combination")]
    WrongPassword,
    #[error("User already exist")]
    UserAlreadyExist,
}
