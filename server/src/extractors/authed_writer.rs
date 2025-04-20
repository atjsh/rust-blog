use crate::env_values;

use super::DatabaseConnection;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct AuthedWriter {
    pub id: i32,
    pub email: String,
    pub description: String,
}

type ErrorResponse = (StatusCode, String);

fn unauthorized() -> ErrorResponse {
    (StatusCode::UNAUTHORIZED, "".to_string())
}

fn service_unavailable() -> ErrorResponse {
    (StatusCode::SERVICE_UNAVAILABLE, "".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

impl<S> FromRequestParts<S> for AuthedWriter
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = ErrorResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(token)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| unauthorized())?;

        let writer_id: i32 = match decode::<Claims>(
            token.token(),
            &DecodingKey::from_secret(std::env::var(env_values::JWT_SECRET).unwrap().as_bytes()),
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
