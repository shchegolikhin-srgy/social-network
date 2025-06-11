
use base64::prelude::*;
use axum::{
    routing::{post, get},
    Router,
    http::StatusCode,
    Json,
};
use std::collections::HashMap;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}
const JWT_SECRET: &[u8] = b"my-secret-key";
pub fn router() -> Router {
    Router::new()
        .route("/login", post(login_handler))
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, StatusCode> {
    if payload.username != "user" ||payload.password !="1234"{
        return Err(StatusCode::UNAUTHORIZED);
    }
    else{
        let claims =  HashMap::<String, String>::from([
            ("sub".to_string(), BASE64_STANDARD.encode(&payload.username)),
            ("username".to_string(), payload.username),
        ]);
        let token = encode(
            &Header::default(),
            &claims,
        &EncodingKey::from_secret(JWT_SECRET)
        ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(AuthResponse { token }))
    }
}