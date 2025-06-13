use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::user::User;

async fn register_user_by_username(
    State(state): State<AppState>,
    user:User
) -> Result<(), sqlx::Error> {
    let username = sqlx::query_scalar!(
        "INSERT INTO users (username, hashed_password) VALUES('$1', '$2');",
        
    )
    .fetch_one(&state.pool) 
    .await?;
    Ok(())
}
async fn register_user_by_email(
    State(state): State<AppState>,
    user:User
) -> Result<(), sqlx::Error> {
    let username = sqlx::query_scalar!(
        "INSERT INTO users (username, hashed_password, email) VALUES('$1', '$2', '$3');",
        1
    )
    .fetch_one(&state.pool) 
    .await?;

    Ok(())
}

async fn register_user_with_role(
    State(state): State<AppState>,
    user:User
) -> Result<(), sqlx::Error> {
    let username = sqlx::query_scalar!(
        "INSERT INTO users (username, hashed_password, role, email) VALUES('$1', '$2', '$3', '$4');",
        1
    )
    .fetch_one(&state.pool) 
    .await?;

    Ok(())
}