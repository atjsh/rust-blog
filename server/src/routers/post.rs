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
    use axum::Json;
    use axum_extra::extract::SignedCookieJar;

    use crate::routers::auth::get_auth_cookie::ACCESS_TOKEN_COOKIE_NAME;

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

    #[derive(Deserialize)]
    pub struct CreatePostBody {
        title: String,
        content: String,
        category_id: i32,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        jar: SignedCookieJar,
        Json(payload): Json<CreatePostBody>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let access_token_cookie = jar.get(ACCESS_TOKEN_COOKIE_NAME);

        let writer_id: i32 = match access_token_cookie {
            Some(cookie) => cookie.value().parse().unwrap(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        let post = sqlx::query_as!(
            GetPostByPostIdRow,
            r#"
            with inserted as (
                insert into post (title, content, written_by_id, category_id)
                values ($1, $2, $3, $4)
                returning *
            )
            select
                p.id,
                p.title,
                p.content,
                p.created_at,
                u.id as "written_by_id!",
                u.email as "written_by_email!",
                c.id as "category_id!",
                c.name as "category_name!"
            from inserted p
            inner join writer u on u.id = p.written_by_id
            inner join category c on c.id = p.category_id
            "#,
            payload.title,
            payload.content,
            writer_id,
            payload.category_id
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        Ok(axum::Json(post.into_nested_post_info()))
    }
}
