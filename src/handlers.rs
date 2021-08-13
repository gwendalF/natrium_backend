use crate::{
    domain::auth::jwt_authentication::{Claims, JwtKey, TokenResponse},
    errors::Result,
    infrastructure::auth,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;

#[get("/temperatures/")]
async fn temp() -> impl Responder {
    "Hello from temperatures"
}

#[post("/google_login/")]
async fn google_login(
    keys: web::Data<JwtKey>,
    pool: web::Data<PgPool>,
    web::Json(google_sub): web::Json<String>,
) -> Result<impl Responder> {
    let existing_user_id = auth::user::check_existing_user(pool.as_ref(), &google_sub).await?;
    if let Some(id) = existing_user_id {
        let claims = Claims::new(id);
        let token = encode(&Header::default(), &claims, &keys.as_ref().encoding).unwrap();
        Ok(HttpResponse::Ok().json(TokenResponse { token }))
    } else {
        Ok(HttpResponse::Ok().body(""))
    }
}

#[get("/google_user/")]
async fn create_google_user(
    web::Json(google_sub): web::Json<String>,
    pool: web::Data<PgPool>,
    keys: web::Data<JwtKey>,
) -> Result<impl Responder> {
    Ok("a")
}
