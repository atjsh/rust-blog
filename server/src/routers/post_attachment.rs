use crate::extractors::AuthedWriter;
use crate::extractors::DatabaseConnection;
use axum::{
    extract::{Multipart, Path},
    http::{header::HeaderMap, StatusCode},
    response::IntoResponse,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod get_post_attachment_by_attachment_id {

    use super::*;

    struct GetPostAttachmentByAttachmentIdRow {
        content: Vec<u8>,
        file_extension: String,
        mimetype: String,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(attachment_id): Path<Uuid>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let post_attachment = sqlx::query_as!(
            GetPostAttachmentByAttachmentIdRow,
            r#"
            select
                pa.content,
                pa.file_extension,
                pa.mimetype
            from post_attachment pa
            where pa.id = $1
            "#,
            attachment_id,
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        // log

        let mut header_map = HeaderMap::new();
        header_map.insert(
            http::header::CONTENT_TYPE,
            post_attachment.mimetype.parse().unwrap(),
        );
        header_map.insert(
            http::header::CONTENT_DISPOSITION,
            format!(
                "inline; filename=\"{}.{}\"",
                attachment_id, post_attachment.file_extension
            )
            .parse()
            .unwrap(),
        );
        header_map.insert(
            http::header::CONTENT_LENGTH,
            post_attachment.content.len().to_string().parse().unwrap(),
        );

        Ok((header_map, post_attachment.content))
    }
}
pub mod create_post_attachment {

    use super::*;

    struct CreatePostAttachmentRow {
        id: Uuid,
        created_at: NaiveDateTime,
    }

    #[derive(Deserialize, Serialize)]
    struct CreatePostAttachmentResponse {
        id: Uuid,
        created_at: NaiveDateTime,
    }

    impl CreatePostAttachmentRow {
        fn into_response(self) -> CreatePostAttachmentResponse {
            CreatePostAttachmentResponse {
                id: self.id,
                created_at: self.created_at,
            }
        }
    }

    pub async fn handler(
        authed_writer: AuthedWriter,
        DatabaseConnection(mut conn): DatabaseConnection,
        mut multipart: Multipart,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let writer_id = authed_writer.id;

        let mut mimetype = None;
        let mut attachment_content = None;
        let mut file_extension = None;

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Error reading form field: {}", e),
            )
        })? {
            let name = field.name().ok_or((
                StatusCode::BAD_REQUEST,
                "Multipart field name not found".to_string(),
            ))?;

            match name {
                "attachment" => {
                    let ct = field.content_type().ok_or((
                        StatusCode::BAD_REQUEST,
                        "Missing content type for attachment".to_string(),
                    ))?;
                    mimetype = Some(ct.to_string());
                    attachment_content = Some(
                        field
                            .bytes()
                            .await
                            .map_err(|err| {
                                (
                                    StatusCode::BAD_REQUEST,
                                    format!("Failed to read attachment bytes: {}", err),
                                )
                            })?
                            .to_vec(),
                    );
                }
                "file_extension" => {
                    file_extension = Some(
                        field
                            .text()
                            .await
                            .map_err(|err| {
                                (
                                    StatusCode::BAD_REQUEST,
                                    format!("Failed to read file_extension: {}", err),
                                )
                            })?
                            .to_string(),
                    );
                }
                _ => {}
            }

            if attachment_content.is_some() && file_extension.is_some() {
                break;
            }
        }

        let content = attachment_content.ok_or((
            StatusCode::BAD_REQUEST,
            "Multipart field 'attachment' not found".to_string(),
        ))?;
        let file_extension = file_extension.ok_or((
            StatusCode::BAD_REQUEST,
            "Multipart field 'file_extension' not found".to_string(),
        ))?;
        let mimetype = mimetype.ok_or((
            StatusCode::BAD_REQUEST,
            "Multipart field 'attachment' not found".to_string(),
        ))?;

        let post_attachment_id = Uuid::now_v7();
        let row = sqlx::query_as!(
            CreatePostAttachmentRow,
            r#"
            insert into post_attachment (id, written_by_id, content, file_extension, mimetype)
            values ($1, $2, $3, $4, $5)
            returning id, created_at
            "#,
            post_attachment_id,
            writer_id,
            content,
            file_extension,
            mimetype,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database query failed: {}", err),
            )
        })?;

        Ok(axum::Json(row.into_response()))
    }
}
