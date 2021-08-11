use super::auth_provider;

pub struct User {
    pub email: String,
    password: Option<String>,
    pub id: i32,
}

impl User {
    pub fn with_password(email: &str, password: Option<&str>) -> Self {
        todo!()
    }
}
