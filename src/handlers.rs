use axum::{
    Json,
    extract::{Path, State},
};

use crate::{AppState, db, error::ApiError, models::Article};

fn lock(state: &AppState) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, ApiError> {
    state.lock().map_err(|_| ApiError::Internal("database lock poisoned".into()))
}

/// GET /articles
pub async fn list_articles(State(state): State<AppState>) -> Result<Json<Vec<Article>>, ApiError> {
    let conn = lock(&state)?;
    Ok(Json(db::all_articles(&conn)?))
}

/// GET /articles/{id}
pub async fn get_article(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<Json<Article>, ApiError> {
    let id: i64 = raw_id
        .parse()
        .ok()
        .filter(|id| *id > 0)
        .ok_or_else(|| ApiError::BadRequest(format!("Invalid article id '{raw_id}'. Expected a positive integer.")))?;

    let conn = lock(&state)?;
    db::article_by_id(&conn, id)?
        .map(Json)
        .ok_or_else(|| ApiError::NotFound(format!("Article with id {id} not found")))
}

pub async fn not_found() -> ApiError {
    ApiError::NotFound("Route not found".into())
}
