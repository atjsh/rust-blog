use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extractors::DatabaseConnection;

#[derive(Deserialize, Serialize)]
struct GetPostRow {
    id: i64,
    title: String,
    content: String,
    created_at: NaiveDateTime,
}

pub async fn get_category_posts(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(category_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(
        GetPostRow,
        "select id, title, content, created_at from post where category_id = $1",
        category_id
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();

    Ok(axum::Json(result))
}
