use axum::{
    Router,
    routing::{post, get},
};
use crate::api::handlers::{login, refresh, register};
pub fn router() -> Router {
    Router::new()
        .route("/login", post(login::login_handler))
        .route("register", post(register::register_handler))
}