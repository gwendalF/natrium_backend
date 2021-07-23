use crate::{
    auth::user::{PostUser, User},
    errors::{AppError, Result},
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
    Ok(HttpResponse::build(StatusCode::OK).json(created_user.uid))
}

#[get("/{user_id}/hubs/")]
async fn hubs_list(pool: web::Data<PgPool>, user_id: web::Path<u64>) -> Result<impl Responder> {
    let db_pool = pool.get_ref();
    Ok("hello")
}
