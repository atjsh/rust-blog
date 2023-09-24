use axum::{http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extractors::DatabaseConnection;

#[derive(Deserialize, Serialize)]
struct GetCategoryRow {
    id: i64,
    name: String,
    created_at: NaiveDateTime,
}

pub async fn get_categories(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(GetCategoryRow, "select id, name, created_at from category")
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    Ok(axum::Json(result))
}
