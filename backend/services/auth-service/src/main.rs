use axum::{
    Router,
    serve,
};
use auth_service::api::routes;
use std::net::SocketAddr;
use auth_service::AppState;

#[tokio::main]
async fn main() {
    let database_url:&'static str = "postgres://postgres:password@localhost/project1users";
    let state = AppState::new(database_url).await.unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));
    let app:Router = Router::new()
        .with_state(state)
        .merge(routes::router());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    serve(listener, app).await.unwrap();
    
}