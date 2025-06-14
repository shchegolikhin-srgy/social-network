use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::user::User;
use sqlx;

pub async fn register_user_by_username(
    State(state): State<AppState>,
    user:User
) -> Result<(), sqlx::Error> {
    println!("Операция в бд!");
    sqlx::query!(
        "INSERT INTO users (username, hashed_password) VALUES($1, $2)",
        user.username,
        user.hashed_password
    )
    .execute(&state.pool) 
    .await?;
    Ok(())
}
