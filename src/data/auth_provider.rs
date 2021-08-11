use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Provider {
    Google,
    Facebook,
}

pub struct GoogleClaims {
    pub aud: String,
    pub iss: String,
    pub exp: String,
    pub email: String,
}
