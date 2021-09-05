use super::auth_types::{email::EmailAddress, password::Password, salt::Salt};

pub struct User {
    email: EmailAddress,
    password: Option<Password>,
    salt: Option<Salt>,
    pub id: i32,
}
