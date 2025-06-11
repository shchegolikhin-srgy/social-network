use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    hashed_password: String,
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}