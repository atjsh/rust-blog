use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::{http::StatusCode, routing::get, Router};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Deserialize, Serialize)]
struct GetPostRow {
    id: i64,
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize)]
struct GetCategoryRow {
    id: i64,
    name: String,
}

async fn root() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let v = "hello world";

    let app = Router::new()
        .route("/", get(root))
        .route("/categories", get(using_connection_extractor))
        .with_state(pool)
        .with_state(v);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(GetCategoryRow, "select id, name from category")
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    Ok(axum::Json(result))
}
