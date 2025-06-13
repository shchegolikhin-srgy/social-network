use base64::prelude::*;
use axum::{
    http::StatusCode,
    Json,
};
use std::collections::HashMap;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
//use crate::models::login::{LoginRequest, AuthResponse};

///Поместит структуры в models

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

const JWT_SECRET: &[u8] = b"my-secret-key";

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, StatusCode> {
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