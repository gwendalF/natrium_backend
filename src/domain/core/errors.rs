use std::num::ParseIntError;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use reqwest::header::ToStrError;
use serde::Serialize;
use thiserror::Error;

use crate::domain::auth::{
    auth_types::key_identifier::{Kid, KidError},
    errors::AuthError,
};
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An unexpected Error occured")]
    ServerError,
    #[error("Environnement error")]
    EnvironnementError,
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Authentication error")]
    AuthenticationError(#[from] AuthError),
}

impl AppError {
    fn name(&self) -> String {
        match self {
            Self::ServerError => "Unexpected error".to_owned(),
            Self::EnvironnementError => "Environnement variable error".to_owned(),
            Self::DatabaseError(_) => "Database error".to_owned(),
            Self::AuthenticationError(e) => match e {
                &AuthError::Credential(_) => "Credential error".to_owned(),
                &AuthError::Email(_) => "Email error".to_owned(),
                &AuthError::Token => "Token error".to_owned(),
                _ => "Unexpected error".to_owned(),
            },
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

impl From<actix_web::Error> for AppError {
    fn from(_: actix_web::Error) -> Self {
        AppError::ServerError
    }
}

impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> Self {
        AppError::ServerError
    }
}

impl From<ToStrError> for AppError {
    fn from(_: ToStrError) -> Self {
        AppError::ServerError
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        AppError::AuthenticationError(AuthError::Token)
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        AppError::ServerError
    }
}

impl From<KidError> for AppError {
    fn from(_: KidError) -> Self {
        AppError::AuthenticationError(AuthError::Kid(KidError::InvalidKid))
    }
}
