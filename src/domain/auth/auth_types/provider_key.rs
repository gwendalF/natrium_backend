use std::collections::HashMap;

use chrono::NaiveDateTime;
use jsonwebtoken::DecodingKey;

#[derive(Clone, Debug)]
pub struct GoogleKeySet {
    pub keys: HashMap<String, DecodingKey<'static>>,
    pub expiration: NaiveDateTime,
}
