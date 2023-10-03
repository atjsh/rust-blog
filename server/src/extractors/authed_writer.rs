use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::SignedCookieJar;
use cookie::Key;
use sqlx::PgPool;

use super::DatabaseConnection;

pub struct AuthedWriter {
    pub id: i32,
    pub email: String,
    pub description: String,
}

fn unauthorized() -> (StatusCode, String) {
    (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthedWriter
where
    S: Send + Sync,
    Key: FromRef<S> + Into<Key>,
    PgPool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = SignedCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .unwrap();

        let cookie = jar.get("access_token");

        let writer_id = match cookie {
            Some(cookie) => cookie.value().parse::<i32>().map_err(|_| unauthorized())?,
            None => return Err(unauthorized()),
        };

        let mut database_connection = DatabaseConnection::from_request_parts(parts, state)
            .await?
            .0;

        sqlx::query_as!(
            AuthedWriter,
            "select id, email, description from writer where id = $1",
            writer_id
        )
        .fetch_one(&mut *database_connection)
        .await
        .map_err(|_| unauthorized())
    }
}
