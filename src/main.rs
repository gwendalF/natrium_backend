use std::sync::Mutex;

use crate::{
    auth::jwt_authentication::{GoogleKey, GoogleKeySet},
    config::Config,
    errors::AppError,
};
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;

mod auth;
mod config;
mod data;
mod errors;
mod handlers;

#[actix_web::main]
async fn main() -> errors::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    let database_url = std::env::var("DATABASE_URL")?;
    let db_pool = PgPool::connect(&database_url).await?;
    let keys_record = sqlx::query!(
        "SELECT modulus, exponent, kid, expiration FROM token_key WHERE provider='google'"
    )
    .fetch_all(&db_pool)
    .await?;
    let keys = keys_record
        .iter()
        .map(|db_record| GoogleKey {
            kid: db_record.kid.to_owned(),
            modulus: db_record.modulus.to_owned(),
            exponent: db_record.exponent.to_owned(),
        })
        .collect::<Vec<_>>();
    let expiration = keys_record[0].expiration;
    println!("{:?}", expiration);
    let key_set = web::Data::new(Mutex::new(GoogleKeySet { keys, expiration }));

    Ok(HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth::jwt_authentication::validator);
        App::new()
            .data(db_pool.clone())
            .app_data(key_set.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "Invalid json",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
            .wrap(middleware::NormalizePath::default())
            .service(handlers::secure_data)
            .service(handlers::create_user)
            .service(handlers::temp)
            .service(
                web::scope("/{user_id}")
                    .wrap(auth)
                    .service(handlers::hubs_list)
                    .service(handlers::rack_temperatures)
                    .service(handlers::add_temperature)
                    .service(handlers::user_detail),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .workers(config.workers)
    .run()
    .await?)
}
