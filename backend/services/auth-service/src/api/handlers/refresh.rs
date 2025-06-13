use base64::prelude::*;
use axum::{
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
//use crate::models::login::{LoginRequest, AuthResponse};

///Поместит структуры в models

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}
pub async fn register_handler(token:String) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}