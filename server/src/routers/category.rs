use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extractors::DatabaseConnection;

pub mod get_categories {
    use super::*;

    #[derive(Deserialize, Serialize)]
    struct GetCategoryRow {
        id: i64,
        name: String,
        created_at: NaiveDateTime,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let result = sqlx::query_as!(GetCategoryRow, "select id, name, created_at from category")
            .fetch_all(&mut *conn)
            .await
            .unwrap();

        Ok(axum::Json(result))
    }
}

pub mod get_category_posts {
    use super::*;

    #[derive(Deserialize, Serialize)]
    struct GetPostByCategoryRow {
        id: i64,
        title: String,
        content: String,
        created_at: NaiveDateTime,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(category_id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let result = sqlx::query_as!(
            GetPostByCategoryRow,
            "select id, title, content, created_at from post where category_id = $1",
            category_id
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        Ok(axum::Json(result))
    }
}
