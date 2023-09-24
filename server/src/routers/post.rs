use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extractors::DatabaseConnection;

pub mod get_post_by_post_id {
    use super::*;

    struct GetPostByPostIdRow {
        id: i64,

        title: String,
        content: String,
        created_at: NaiveDateTime,

        written_by_id: i64,
        written_by_email: String,

        category_id: i64,
        category_name: String,
    }

    #[derive(Deserialize, Serialize)]
    struct GetPostByPostIdResponse {
        id: i64,

        title: String,
        content: String,
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

    impl GetPostByPostIdRow {
        fn into_nested_post_info(self) -> GetPostByPostIdResponse {
            GetPostByPostIdResponse {
                id: self.id,
                title: self.title,
                content: self.content,
                created_at: self.created_at,
                written_by: GetPostByPostIdResponseWrittenBy {
                    id: self.written_by_id,
                    email: self.written_by_email,
                },
                category: GetPostByPostIdResponseCategory {
                    id: self.category_id,
                    name: self.category_name,
                },
            }
        }
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(post_id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let post = sqlx::query_as!(
            GetPostByPostIdRow,
            r#"
            select
                p.id,
                p.title,
                p.content,
                p.created_at,
                u.id as "written_by_id!",
                u.email as "written_by_email!",
                c.id as "category_id!",
                c.name as "category_name!"
            from post p
            inner join writer u on u.id = p.written_by_id
            inner join category c on c.id = p.category_id
            where p.id = $1
            "#,
            post_id
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap()
        .into_nested_post_info();

        Ok(axum::Json(post))
    }
}

pub mod create_post {
    use super::*;

    #[derive(Deserialize)]
    struct CreatePostBody {
        title: String,
        content: String,
        category_id: i64,
    }
}
