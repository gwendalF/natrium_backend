use std::sync::{Arc, Mutex};

use crate::domain::auth::ports::ProviderKeySet;

pub enum AuthProvider {
    Google(ProviderKeySet),
    Facebook,
}
