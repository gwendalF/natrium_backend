use std::collections::HashMap;

use jsonwebtoken::DecodingKey;

use super::key_identifier::Kid;

#[derive(Debug)]
pub enum AuthProvider {
    Google,
    Facebook,
}

#[derive(Debug, Clone)]
pub struct ProviderKeySet {
    pub keys: HashMap<Kid, DecodingKey<'static>>,
    pub expiration: chrono::NaiveDateTime,
}
