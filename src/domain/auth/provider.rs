use super::jwt_authentication::GoogleKeySet;

pub enum AuthProvider {
    Google(GoogleKeySet),
    Facebook,
}
