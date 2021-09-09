use crate::domain::auth::auth_types::credential::{ClearCredential, Credential};
use crate::domain::auth::auth_types::provider::AuthProvider;
use crate::domain::auth::ports::{IAuthService, ProviderKeySet, Token};
use crate::Result;
use actix_web::{error, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::convert::TryFrom;
use std::sync::Mutex;

pub fn configure<T: 'static + IAuthService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route(
        "/register_credential",
        web::post().to(register_credential::<T>),
    );
    cfg.route("/register_google/", web::post().to(register_provider::<T>));
    cfg.route("/login/", web::post().to(login_credential::<T>));
    cfg.route("/login_google/", web::post().to(login_provider::<T>));
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _req| {
        error::InternalError::from_response(
            "Invalid json",
            HttpResponse::BadRequest()
                .content_type("application/json")
                .body(format!(r#"{{"error":"{}"#, err)),
        )
        .into()
    }));
}

async fn register_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<ClearCredential>,
) -> Result<web::Json<String>> {
    let credential = Credential::try_from(body)?;
    Ok(web::Json(service.register_credential(&credential).await?.0))
}

async fn login_credential<T: IAuthService>(
    service: web::Data<T>,
    web::Json(body): web::Json<ClearCredential>,
) -> Result<web::Json<String>> {
    Ok(web::Json(service.login_credential(&body).await?.0))
}

async fn register_provider<T: IAuthService>(
    service: web::Data<T>,
    token: BearerAuth,
    key_set: web::Data<Mutex<ProviderKeySet>>,
) -> Result<web::Json<String>> {
    let token = Token(token.token().to_owned());
    let provider = AuthProvider::Google;
    Ok(web::Json(
        service
            .register_provider(&token, &provider, key_set.get_ref())
            .await?
            .0,
    ))
}

async fn login_provider<T: IAuthService>(
    service: web::Data<T>,
    token: BearerAuth,
    key_set: web::Data<Mutex<ProviderKeySet>>,
) -> Result<web::Json<String>> {
    let token = Token(token.token().to_owned());
    let provider = AuthProvider::Google;
    Ok(web::Json(
        service
            .login_provider(&token, &provider, key_set.get_ref())
            .await?
            .0,
    ))
}
