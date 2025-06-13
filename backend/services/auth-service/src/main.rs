use axum::{
    Router,
    serve,
    extract::State,
};
use serde_json::{Value, json};
use api::routes;
use sqlx::PgPool;
use std::net::SocketAddr;
use auth_service::AppState;
mod api;
mod infrastructure;

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:password@localhost/project1users";
    let state = AppState::new(database_url).await.unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));
    let app:Router = Router::new()
        .with_state(state)
        .merge(routes::router());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    serve(listener, app).await.unwrap();
    
}