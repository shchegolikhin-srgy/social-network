use base64::prelude::*;
use axum::{
    routing::{post, get},
    Router,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::models::login::{RegisterRequest, AuthResponse};

///Поместит структуры в models


pub async fn register_handler(Json(payload): Json<RegisterRequest>) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}