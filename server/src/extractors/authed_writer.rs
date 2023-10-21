use crate::env_values;

use super::DatabaseConnection;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct AuthedWriter {
    pub id: i32,
    pub email: String,
    pub description: String,
}

type CookieAssigningResponse = (StatusCode, String);

fn unauthorized() -> CookieAssigningResponse {
    (StatusCode::UNAUTHORIZED, "".to_string())
}

fn service_unavailable() -> CookieAssigningResponse {
    (StatusCode::SERVICE_UNAVAILABLE, "".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthedWriter
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = CookieAssigningResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(token)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| unauthorized())?;

        println!("token: {}", token.token());

        let writer_id: i32 = match decode::<Claims>(
            token.token(),
            &DecodingKey::from_secret(std::env::var(env_values::COOKIE_SECRET).unwrap().as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        ) {
            Ok(token_data) => token_data
                .claims
                .sub
                .parse()
                .ok()
                .ok_or_else(unauthorized)?,
            Err(_) => return Err(unauthorized()),
        };

        let mut database_connection = DatabaseConnection::from_request_parts(parts, state)
            .await
            .map_err(|_| service_unavailable())?
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
