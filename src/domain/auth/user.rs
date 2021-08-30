use super::value_object::{email::EmailAddress, password::Password};

pub struct User {
    email: EmailAddress,
    password: Option<Password>,
    pub id: i32,
}
