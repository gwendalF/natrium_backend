use std::sync::Mutex;

use crate::domain::auth::ports::ProviderKeySet;

pub enum AuthProvider {
    Google(Mutex<ProviderKeySet>),
    Facebook,
}
