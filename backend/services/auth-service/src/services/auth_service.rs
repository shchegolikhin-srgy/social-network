use axum::extract::State;
use sqlx;
use crate::core::app_state::AppState;
async fn get_user(
    State(state): State<AppState>,
) -> Result<String, String> {
    
    Ok(format!("User: "))
}