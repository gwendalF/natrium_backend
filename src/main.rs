use crate::config::Config;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
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
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
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
            .service(
                web::scope("/{user_id}")
                    .service(handlers::hubs_list)
                    .service(handlers::rack_temperatures)
                    .service(handlers::add_temperature),
            )
            .service(handlers::secure_data)
            .service(handlers::create_user)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await?;
    Ok(())
}
