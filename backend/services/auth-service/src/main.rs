use auth_service::models::user::User;
use axum::extract::State;
use axum::{
    Router,
    serve,
};
use auth_service::api::routes;
use auth_service::AppState;
use auth_service::services::register_service;
use auth_service::core::config::Settings;
use sqlx::Error;

#[tokio::main]
async fn main() {
    let settings = Settings::new().await.unwrap();

    let state = AppState::new(&settings.database_url).await.unwrap();
    
    let app:Router = Router::new()
        .with_state(state.clone())
        .merge(routes::router());
    let listener = tokio::net::TcpListener::bind(&settings.addr).await.unwrap();
    use auth_service::services::register_service;
    let conn:Result<(), Error> = register_service::register_user_by_username(State(state.clone()), User{
        hashed_password: String::from("password"),
        username: String::from("pidor@"),
        email: String::from("example@"),
    }).await;
    match conn {
        Ok(())=>println!("Ok"),
        _ =>println!("Failed!!!!"),
    }
    

    serve(listener, app).await.unwrap();
}