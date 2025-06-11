use axum::{
    Router,
    routing::{get, post},
    middleware,
    extract::Request,
    response::Response,
    Json,
};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
}