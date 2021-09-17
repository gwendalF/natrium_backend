use actix_web::{cookie::Cookie, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};

use super::claims::Claims;
use crate::domain::auth::auth_types::claims::REFRESH_TOKEN_DURATION;
use crate::Result;

pub enum TokenType {
    RefreshToken,
    AccessToken,
}

#[derive(Debug, serde::Serialize, PartialEq, Eq)]
pub struct Token(pub String);

#[derive(Debug, serde::Serialize, PartialEq, Eq)]
pub struct AuthToken {
    pub refresh_token: Token,
    pub access_token: Token,
    pub expiration: usize,
}

impl AuthToken {
    pub fn new(
        user_id: i32,
        access_key: &EncodingKey,
        refresh_key: &EncodingKey,
    ) -> Result<AuthToken> {
        let access_claims = Claims::new(user_id, TokenType::AccessToken);
        let refresh_claims = Claims::new(user_id, TokenType::RefreshToken);
        let expiration = access_claims.exp;
        let access_token = Token(encode(&Header::default(), &access_claims, access_key)?);
        let refresh_token = Token(encode(&Header::default(), &refresh_claims, refresh_key)?);
        Ok(AuthToken {
            refresh_token,
            access_token,
            expiration,
        })
    }
}

impl From<AuthToken> for HttpResponse {
    fn from(token: AuthToken) -> Self {
        let when = time::OffsetDateTime::now_utc() + time::Duration::days(REFRESH_TOKEN_DURATION);
        let cookie = Cookie::build("refresh_token", &token.refresh_token.0)
            .path("/refresh_token/")
            .secure(true)
            .expires(when)
            .http_only(true)
            .finish();
        #[derive(serde::Serialize)]
        struct AccessToken {
            access_token: String,
            expiration: usize,
        }
        HttpResponse::build(actix_web::http::StatusCode::OK)
            .cookie(cookie)
            .json(AccessToken {
                access_token: token.access_token.0,
                expiration: token.expiration,
            })
    }
}
