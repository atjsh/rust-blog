use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extractors::{AuthedWriter, DatabaseConnection};

pub mod get_writer_by_writer_id {
    use super::*;

    struct GetWriterByWriterIdRow {
        id: i64,
        email: String,
        description: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct GetWriterByWriterIdResponse {
        id: i64,
        email: String,
        description: String,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(writer_id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        sqlx::query_as!(
            GetWriterByWriterIdRow,
            r#"
            SELECT id, email, description
            FROM writer
            WHERE id = $1
            "#,
            writer_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map(|row| {
            row.map(|row| {
                Json(GetWriterByWriterIdResponse {
                    id: row.id,
                    email: row.email,
                    description: row.description,
                })
            })
            .ok_or((StatusCode::NOT_FOUND, "writer not found".to_string()))
        })
        .unwrap()
    }
}

pub mod get_posts_by_writer_id {
    use super::*;

    struct GetPostByWriterRow {
        id: i64,

        title: String,
        created_at: NaiveDateTime,

        written_by_id: i64,
        written_by_email: String,

        category_id: i64,
        category_name: String,
    }

    #[derive(Deserialize, Serialize)]
    struct GetPostByWriterResponse {
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

    impl From<GetPostByWriterRow> for GetPostByWriterResponse {
        fn from(row: GetPostByWriterRow) -> Self {
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
        Path(writer_id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let result = sqlx::query_as!(
            GetPostByWriterRow,
            r#"
            SELECT post.id, post.title, post.created_at, post.written_by_id, writer.email AS written_by_email, post.category_id, category.name AS category_name
            FROM post
            INNER JOIN writer ON post.written_by_id = writer.id
            INNER JOIN category ON post.category_id = category.id
            WHERE post.written_by_id = $1
            order by post.created_at desc
            "#,
            writer_id
        )
        .fetch_all(&mut *conn)
        .await
        .unwrap();

        Ok(axum::Json(
            result
                .into_iter()
                .map(|row| row.into())
                .collect::<Vec<GetPostByWriterResponse>>(),
        ))
    }
}

pub mod update_writer {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct UpdateWriterRequest {
        email: Option<String>,
        description: Option<String>,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        authed_writer: AuthedWriter,
        Json(body): Json<UpdateWriterRequest>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        sqlx::query!(
            r#"
            UPDATE writer
            SET email = COALESCE($1, email), description = COALESCE($2, description)
            WHERE id = $3
            "#,
            body.email,
            body.description,
            authed_writer.id
        )
        .execute(&mut *conn)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "can't update writer".to_string(),
            )
        })?;

        Ok(StatusCode::OK)
    }
}
