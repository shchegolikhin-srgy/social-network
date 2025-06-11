use axum::{
    Router,
    serve,
    routing::{post, get},
    response::Json,
    body::Body,
};
use std::net::SocketAddr;
use serde_json::{Value, json};
use api::routes;

mod api;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3003));
    let app:Router = Router::new()
        .route("/fetch_recommendations", get(get_recommendations))
        .merge(routes::router());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    serve(listener, app).await.unwrap();
}

async fn get_recommendations()->Json<Value>{
    return Json(json!([{"title":"TITLE", "author":"Sergey"}]))
}