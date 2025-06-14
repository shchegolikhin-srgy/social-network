use axum::{
    Router,
    serve,
};
use auth_service::api::routes;
use auth_service::AppState;
use auth_service::services::register_service;
use auth_service::core::config;

#[tokio::main]
async fn main() {
    let database_url:&'static str = "postgres://postgres:password@localhost/project1users";

    let settings = config::Settings::load_config()?;

    let state = AppState::new(settings.database_url).await.unwrap();
    
    let app:Router = Router::new()
        .with_state(state)
        .merge(routes::router());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    serve(listener, app).await.unwrap();
    
}