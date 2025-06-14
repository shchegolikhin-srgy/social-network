use base64::prelude::*;
use axum::{
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::models::login::{LoginRequest, AuthResponse};


pub async fn register_handler(token:String) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}