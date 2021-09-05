use crate::domain::auth::auth_types::provider_key::GoogleKeySet;
use crate::Result;
use crate::{
    application,
    domain::{
        self,
        auth::jwt_authentication::{AppKey, Claims, TokenResponse},
    },
    infrastructure::auth,
};
use actix_web::{get, post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use sqlx::PgPool;
#[get("/temperatures/")]
async fn temp() -> impl Responder {
    "Hello from temperatures"
}

#[post("/google_login/")]
async fn google_login(
    pool: web::Data<PgPool>,
    provider_key: web::Data<GoogleKeySet>,
    jwt_key: web::Data<AppKey>,
    token: BearerAuth,
) -> Result<impl Responder> {
    Ok("Todo!")
}

#[get("/google_user/")]
async fn create_google_user(
    web::Json(_google_sub): web::Json<String>,
    _pool: web::Data<PgPool>,
    _keys: web::Data<AppKey>,
) -> Result<impl Responder> {
    Ok("a")
}
