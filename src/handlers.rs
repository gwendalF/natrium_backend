use crate::{
    auth::user::{PostUser, User},
    data::{
        hub::Hub,
        temperature::{Temperature, UserTemperature},
    },
    errors::Result,
};
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_permissions;
use sqlx::PgPool;

#[get("/temperatures/")]
async fn temp() -> impl Responder {
    "Hello from temperatures"
}

#[get("/hygrometrie/")]
async fn hygro() -> impl Responder {
    "Hello from hygrometrie"
}

#[get("/secure/")]
#[has_permissions("ROLE_ADMIN")]
async fn secure_data() -> impl Responder {
    "Secured data"
}

#[post("/user/")]
async fn create_user(pool: web::Data<PgPool>, user: web::Json<PostUser>) -> Result<impl Responder> {
    let db_pool = pool.get_ref();
    let created_user = User::create(db_pool, &user.email, &user.password).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(created_user.id))
}

#[get("/hubs/")]
async fn hubs_list(pool: web::Data<PgPool>, user_id: web::Path<(i64,)>) -> Result<impl Responder> {
    let db_pool = pool.get_ref();
    let hubs_list = Hub::get_all(db_pool, user_id.into_inner().0).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(hubs_list))
}

#[get("/{localisation_id}/temperatures/")]
async fn rack_temperatures(
    pool: web::Data<PgPool>,
    path: web::Path<(i64, i64)>,
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
    user_id: web::Path<(i64,)>,
) -> Result<impl Responder> {
    Ok(format!("hello user {}", user_id.0 .0))
}
