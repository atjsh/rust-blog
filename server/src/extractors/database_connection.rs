use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sqlx::postgres::{PgConnectOptions, PgConnection};
use sqlx::ConnectOptions;

fn internal_error() -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
}

pub struct DatabaseConnection(pub sqlx::postgres::PgConnection);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgConnectOptions: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let connection_options: PgConnectOptions = PgConnectOptions::from_ref(state);

        let conn: PgConnection = connection_options
            .connect()
            .await
            .map_err(|_| internal_error())?;

        Ok(Self(conn))
    }
}
