use axum::extract::State;

use crate::core::app_state::AppState;
async fn get_user(
    State(state): State<AppState>,
) -> Result<String, String> {
    
    Ok(format!("User: "))
}