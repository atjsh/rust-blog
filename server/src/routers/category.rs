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

pub mod get_category {
    use super::*;

    #[derive(Deserialize, Serialize)]
    struct GetCategoryRow {
        id: i64,
        name: String,
        created_at: NaiveDateTime,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(category_id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let result = sqlx::query_as!(
            GetCategoryRow,
            "select id, name, created_at from category where id = $1",
            category_id
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        Ok(axum::Json(result))
    }
}

pub mod get_category_posts {
    use super::*;

    struct GetPostByCategoryRow {
        id: i64,

        title: String,
        created_at: NaiveDateTime,

        written_by_id: i64,
        written_by_email: String,

        category_id: i64,
        category_name: String,
    }

    #[derive(Deserialize, Serialize)]
    struct GetPostByCategoryResponse {
        id: i64,

        title: String,
        created_at: NaiveDateTime,

        written_by: GetPostByPostIdResponseWrittenBy,

        category: GetPostByPostIdResponseCategory,
    }

    #[derive(Deserialize, Serialize)]
    struct GetPostByPostIdResponseWrittenBy {
        id: i64,
        email: String,
    }

    #[derive(Deserialize, Serialize)]
    struct GetPostByPostIdResponseCategory {
        id: i64,
        name: String,
    }

    impl From<GetPostByCategoryRow> for GetPostByCategoryResponse {
        fn from(row: GetPostByCategoryRow) -> Self {
            Self {
                id: row.id,

                title: row.title,
                created_at: row.created_at,

                written_by: GetPostByPostIdResponseWrittenBy {
                    id: row.written_by_id,
                    email: row.written_by_email,
                },

                category: GetPostByPostIdResponseCategory {
                    id: row.category_id,
                    name: row.category_name,
                },
            }
        }
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(category_id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let result = sqlx::query_as!(
            GetPostByCategoryRow,
            r#"
            select
                post.id,
                post.title,
                post.created_at,
                post.written_by_id,
                writer.email as written_by_email,
                post.category_id,
                category.name as category_name
            from post
            inner join writer on post.written_by_id = writer.id
            inner join category on post.category_id = category.id
            where post.category_id = $1 
            and post.private = false
            order by post.created_at desc
            "#,
            category_id
        )
        .fetch_all(&mut *conn)
        .await
        .unwrap();

        Ok(axum::Json(
            result
                .into_iter()
                .map(|row| row.into())
                .collect::<Vec<GetPostByCategoryResponse>>(),
        ))
    }
}
