use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An unexpected Error occured")]
    ServerError,
    #[error("You are not allowed to access")]
    PermissionDenied,
    #[error("Ressource was not found")]
    NotFoundError,
    #[error("Ressource already exist")]
    AlreadyExist,
    #[error("Environnement error")]
    EnvironnementError,
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
}

impl AppError {
    fn name(&self) -> String {
        match self {
            Self::ServerError => "Unexpected error".to_owned(),
            Self::NotFoundError => "Not found".to_owned(),
            Self::AlreadyExist => "Already exist".to_owned(),
            Self::PermissionDenied => "Access denied".to_owned(),
            Self::EnvironnementError => "Environnement variable error".to_owned(),
            Self::DatabaseError(_) => "Database error".to_owned(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFoundError => StatusCode::NOT_FOUND,
            Self::PermissionDenied => StatusCode::FORBIDDEN,
            Self::AlreadyExist => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(ErrorResponse {
            code: status_code.as_u16(),
            error: self.name(),
            message: self.to_string(),
        })
    }
}

impl From<config::ConfigError> for AppError {
    fn from(_: config::ConfigError) -> Self {
        AppError::EnvironnementError
    }
}

impl From<std::env::VarError> for AppError {
    fn from(_: std::env::VarError) -> Self {
        AppError::EnvironnementError
    }
}

impl From<argon2::Error> for AppError {
    fn from(_: argon2::Error) -> Self {
        AppError::ServerError
    }
}

impl From<std::io::Error> for AppError {
    fn from(_: std::io::Error) -> Self {
        AppError::ServerError
    }
}
