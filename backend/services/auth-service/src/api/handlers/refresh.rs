use base64::prelude::*;
use axum::{
    routing::{post, get},
    Router,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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

const JWT_SECRET: &[u8] = b"my-secret-key";

pub async fn register_handler(token:String) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}