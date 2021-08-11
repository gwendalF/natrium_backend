use crate::{
    auth::jwt_authentication::{Claims, JwtKey, TokenResponse},
    data::{
        hub::Hub,
        temperature::{Temperature, UserTemperature},
    },
    errors::Result,
    infrastructure::user::check_existing_user,
};
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_permissions;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;

#[get("/temperatures/")]
async fn temp() -> impl Responder {
    "Hello from temperatures"
}

#[get("/secure/")]
#[has_permissions("ROLE_ADMIN")]
async fn secure_data() -> impl Responder {
    "Secured data"
}

#[get("/hubs/")]
async fn hubs_list(pool: web::Data<PgPool>, user_id: web::Path<(i32,)>) -> Result<impl Responder> {
    let db_pool = pool.get_ref();
    let hubs_list = Hub::get_all(db_pool, user_id.into_inner().0).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(hubs_list))
}

#[get("/{localisation_id}/temperatures/")]
async fn rack_temperatures(
    pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder> {
    let db_pool = pool.get_ref();
    let (user_id, localisation_id) = path.into_inner();
    let temps = Temperature::get_by_rack(db_pool, localisation_id).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(temps))
}

#[post("/temperature/")]
async fn add_temperature(
    pool: web::Data<PgPool>,
    temperature: web::Json<UserTemperature>,
) -> Result<impl Responder> {
    let db_pool = pool.get_ref();
    let temperature = temperature.into_inner();
    let inserted = Temperature::add(db_pool, temperature).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(inserted))
}

#[get("/")]
async fn user_detail(
    pool: web::Data<PgPool>,
    user_id: web::Path<(i32,)>,
) -> Result<impl Responder> {
    Ok(format!("hello user {}", user_id.0 .0))
}

#[post("/google_login/")]
async fn google_login(
    keys: web::Data<JwtKey>,
    pool: web::Data<PgPool>,
    web::Json(google_sub): web::Json<String>,
) -> Result<impl Responder> {
    let existing_user_id = check_existing_user(&google_sub, pool.as_ref()).await?;
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
