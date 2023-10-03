use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::SignedCookieJar;
use cookie::{Cookie, Key};
use sqlx::PgPool;

use crate::routers::auth::get_auth_cookie::ACCESS_TOKEN_COOKIE_NAME;

use super::DatabaseConnection;

pub struct AuthedWriter {
    pub id: i32,
    pub email: String,
    pub description: String,
}

fn unauthorized(jar: SignedCookieJar) -> (StatusCode, SignedCookieJar, String) {
    (StatusCode::UNAUTHORIZED, jar, "Unauthorized".to_string())
}

fn service_unavailable(jar: SignedCookieJar) -> (StatusCode, SignedCookieJar, String) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        jar,
        "Service Temporarily Unavailable".to_string(),
    )
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthedWriter
where
    S: Send + Sync,
    Key: FromRef<S> + Into<Key>,
    PgPool: FromRef<S>,
{
    type Rejection = (StatusCode, SignedCookieJar<Key>, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar: SignedCookieJar = SignedCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .unwrap();

        let writer_id: i32 = jar
            .get(ACCESS_TOKEN_COOKIE_NAME)
            .and_then(|cookie| cookie.value().parse().ok())
            .ok_or_else(|| {
                unauthorized(jar.clone().remove(Cookie::named(ACCESS_TOKEN_COOKIE_NAME)))
            })?;

        let mut database_connection = DatabaseConnection::from_request_parts(parts, state)
            .await
            .map_err(|_| service_unavailable(jar.clone()))?
            .0;

        sqlx::query_as!(
            AuthedWriter,
            "select id, email, description from writer where id = $1",
            writer_id
        )
        .fetch_one(&mut *database_connection)
        .await
        .map_err(|_| unauthorized(jar.remove(Cookie::named(ACCESS_TOKEN_COOKIE_NAME))))
    }
}
