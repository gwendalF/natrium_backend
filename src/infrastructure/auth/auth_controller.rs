use std::sync::Mutex;

use crate::domain::auth::auth_types::credential::{ClearCredential, Credential};
use crate::domain::auth::auth_types::provider::AuthProvider;
use crate::domain::auth::ports::{IAuthService, ProviderKeySet, Token};
use crate::Result;
use actix_web::{post, web, HttpResponse};

pub fn configure<T: 'static + IAuthService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route(
        "/register_credential",
        web::post().to(register_credential::<T>),
    );
    cfg.route("/register_google/", web::post().to(register_provider::<T>));
    cfg.route("/login/", web::post().to(login_credential::<T>));
    cfg.route("/login_google/", web::post().to(login_provider::<T>));
}

async fn register_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<Credential>,
) -> Result<web::Json<String>> {
    Ok(web::Json(service.register_credential(&body).await?.0))
}

async fn login_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<ClearCredential>,
) -> Result<web::Json<String>> {
    Ok(web::Json(service.login_credential(&body).await?.0))
}

async fn register_provider<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<String>,
    key_set: web::Data<Mutex<ProviderKeySet>>,
) -> Result<web::Json<String>> {
    let token = Token(body);
    Ok(web::Json(
        service
            .register_provider(&token, &AuthProvider::Google, &key_set)
            .await?
            .0,
    ))
}

async fn login_provider<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<String>,
    key_set: web::Data<Mutex<ProviderKeySet>>,
) -> Result<web::Json<String>> {
    let token = Token(body);
    Ok(web::Json(
        service
            .login_provider(&token, &AuthProvider::Google, &key_set)
            .await?
            .0,
    ))
}
