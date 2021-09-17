use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub struct AccessKey {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey<'static>,
}

#[derive(Debug, Clone)]
pub struct RefreshKey {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey<'static>,
}
