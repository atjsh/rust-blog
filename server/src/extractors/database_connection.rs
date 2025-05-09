use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sqlx::PgPool;

fn internal_error() -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
}

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(|_| internal_error())?;

        Ok(Self(conn))
    }
}
