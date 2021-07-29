use crate::{auth::jwt_authentication::GoogleKey, config::Config};
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::DecodingKey;
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
    let key =
        sqlx::query!("SELECT modulus, exponent, expiration FROM token_key WHERE name='google'")
            .fetch_one(&db_pool)
            .await?;
    let google_key = GoogleKey {
        key: DecodingKey::from_rsa_components(&key.modulus, &key.exponent),
        expiration: key.expiration,
    };
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth::jwt_authentication::validator);
        App::new()
            .data(db_pool.clone())
            .data(google_key.clone())
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
    .run()
    .await?;
    Ok(())
}
