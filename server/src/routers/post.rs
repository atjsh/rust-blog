use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extractors::{AuthedWriter, DatabaseConnection};

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(i16)]
enum PostAd {
    NoAd = 0,
    CoupangAd = 1,
}

impl From<i16> for PostAd {
    fn from(value: i16) -> Self {
        serde_json::from_value(serde_json::json!(value)).unwrap_or(PostAd::NoAd)
    }
}

pub mod get_post_by_post_id {
    use super::*;

    struct GetPostByPostIdRow {
        id: i64,

        title: String,
        content: String,
        content_type: String,
        private: bool,
        ad: PostAd,
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
        content_type: String,
        private: bool,
        ad: PostAd,
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
                content_type: self.content_type,
                private: self.private,
                ad: self.ad,
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
                p.content_type,
                p.private,
                p.ad,
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

    use super::*;

    struct GetPostByPostIdRow {
        id: i64,

        title: String,
        content: String,
        content_type: String,
        private: bool,
        ad: PostAd,
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
        content_type: String,
        private: bool,
        ad: PostAd,
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
                content_type: self.content_type,
                private: self.private,
                ad: self.ad,
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
        content_type: String,
        is_private: bool,
        ad: i16,
        category_id: i32,
    }

    pub async fn handler(
        authed_writer: AuthedWriter,
        DatabaseConnection(mut conn): DatabaseConnection,
        Json(payload): Json<CreatePostBody>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let writer_id = authed_writer.id;

        if payload.content_type != "html" && payload.content_type != "md" {
            return Err(StatusCode::BAD_REQUEST);
        }

        let post = sqlx::query_as!(
            GetPostByPostIdRow,
            r#"
            with inserted as (
                insert into post (title, content, content_type, private, ad, written_by_id, category_id)
                values ($1, $2, $3, $4, $5, $6, $7)
                returning *
            )
            select
                p.id,
                p.title,
                p.content,
                p.content_type,
                p.private,
                p.ad,
                p.created_at,
                u.id as "written_by_id",
                u.email as "written_by_email",
                c.id as "category_id",
                c.name as "category_name"
            from inserted p
            inner join writer u on u.id = p.written_by_id
            inner join category c on c.id = p.category_id
            "#,
            payload.title,
            payload.content,
            payload.content_type,
            payload.is_private,
            payload.ad,
            writer_id,
            payload.category_id
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        Ok(axum::Json(post.into_nested_post_info()))
    }
}

pub mod update_post {
    use axum::Json;

    use super::*;

    struct GetPostByPostIdRow {
        id: i64,

        title: String,
        content: String,
        content_type: String,
        private: bool,
        ad: PostAd,
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
        content_type: String,
        private: bool,
        ad: PostAd,
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
                content_type: self.content_type,
                private: self.private,
                ad: self.ad,
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
    pub struct UpdatePostBody {
        title: String,
        content: String,
        content_type: String,
        is_private: bool,
        ad: i16,
        category_id: i32,
    }

    pub async fn handler(
        authed_writer: AuthedWriter,
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(post_id): Path<i32>,
        Json(payload): Json<UpdatePostBody>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let writer_id = authed_writer.id;

        if payload.content_type != "html" && payload.content_type != "md" {
            return Err(StatusCode::BAD_REQUEST);
        }

        let post = sqlx::query_as!(
            GetPostByPostIdRow,
            r#"
            with updated as (
                update post
                set title = $3, content = $4, content_type = $5, private = $6, category_id = $7, ad = $8
                where id = $1 and written_by_id = $2
                returning *
            )
            select
                p.id,
                p.title,
                p.content,
                p.content_type,
                p.private,
                p.ad,
                p.created_at,
                u.id as "written_by_id",
                u.email as "written_by_email",
                c.id as "category_id",
                c.name as "category_name"
            from updated p
            inner join writer u on u.id = p.written_by_id
            inner join category c on c.id = p.category_id
            "#,
            post_id,
            writer_id,
            payload.title,
            payload.content,
            payload.content_type,
            payload.is_private,
            payload.category_id,
            payload.ad
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        Ok(axum::Json(post.into_nested_post_info()))
    }
}

pub mod delete_post {
    use super::*;

    pub async fn handler(
        authed_writer: AuthedWriter,
        DatabaseConnection(mut conn): DatabaseConnection,
        Path(post_id): Path<i32>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let writer_id = authed_writer.id;

        sqlx::query!(
            r#"
            delete from post
            where id = $1 and written_by_id = $2
            "#,
            post_id,
            writer_id
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        Ok(StatusCode::NO_CONTENT)
    }
}
