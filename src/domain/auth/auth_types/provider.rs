use crate::domain::auth::ports::ProviderKeySet;

pub enum AuthProvider {
    Google(ProviderKeySet),
    Facebook,
}
