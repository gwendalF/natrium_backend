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
// #[post("/user")]
// async fn create_user(pool: web::Data<PgPool>, email: String, password: String) -> impl Responder {
//     // Check if user already exist
//     let db_pool = pool.get_ref();
//     let previous_user = sqlx::query!("SELECT email FROM users WHERE email = $1", email)
//         .fetch_optional(db_pool)
//         .await
//         .map_err(|err| NatriumError {
//             message: None,
//             cause: Some(err.to_string()),
//             error_type: ErrorType::DatabaseError,
//         })?;
//     match previous_user {
//         Some(_) => Err(NatriumError {
//             message: Some("Email already in use".to_owned()),
//             cause: None,
//             error_type: ErrorType::DatabaseError,
//         }),
//         None => {
//             let new_user = sqlx::query!(
//                 "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING uid",
//                 &email,
//                 &password
//             )
//             .fetch_one(db_pool)
//             .await
//             .map_err(|err| NatriumError {
//                 message: Some("Cannot create user".to_owned()),
//                 cause: Some(err.to_string()),
//                 error_type: ErrorType::DatabaseError,
//             })?;
//             Ok(HttpResponse::Ok().json(User::new(new_user.uid, &email, &password)))
//         }
//     }
// }
