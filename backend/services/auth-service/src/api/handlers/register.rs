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

pub async fn register_handler(Json(payload): Json<RegisterRequest>) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}