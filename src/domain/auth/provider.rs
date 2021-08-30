use super::ports::ProviderKeySet;

pub enum AuthProvider {
    Google(ProviderKeySet),
    Facebook,
}
