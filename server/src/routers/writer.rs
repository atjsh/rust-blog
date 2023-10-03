use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
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

pub mod update_writer {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct UpdateWriterRequest {
        email: String,
        description: String,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        authed_writer: AuthedWriter,
        Json(body): Json<UpdateWriterRequest>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        sqlx::query!(
            r#"
            UPDATE writer
            SET email = $1, description = $2
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

        Ok(axum::Json(()))
    }
}
