use crate::core::app_state::AppState;
use axum::extract::State;

async fn get_connection_for_quick_task(
    State(state): State<AppState>
) -> Result<(), sqlx::Error> {
    let username = sqlx::query_scalar!(
        "SELECT username FROM users WHERE id = $1",
        1
    )
    .fetch_one(&state.pool) 
    .await?;
    Ok(())
}