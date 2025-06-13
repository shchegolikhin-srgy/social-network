use axum::{
    Router,
    routing::{post, get},
};
use crate::api::handlers::login;
pub fn router() -> Router {
    Router::new()
        .route("/login", post(login::login_handler))
}