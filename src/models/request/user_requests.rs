use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: String
}