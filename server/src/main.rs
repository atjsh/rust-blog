mod env_values;
mod extractors;
mod repositories;
mod routers;

use axum::{
    extract::FromRef,
    http::HeaderValue,
    routing::{delete, get, patch, post, put},
    Router,
};
use axum_extra::extract::cookie::Key;
use http::{
    header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD, CONTENT_TYPE,
    },
    Method,
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[derive(Clone, FromRef)]
pub struct AppState {
    cookie_secret_key: Key,
    pg_pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    let db_connection_str = std::env::var(env_values::DATABASE_URL).unwrap();

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let cookit_secret_key_string = std::env::var(env_values::COOKIE_SECRET).unwrap();

    let state = AppState {
        cookie_secret_key: Key::from(cookit_secret_key_string.as_bytes()),
        pg_pool,
    };

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::OPTIONS,
            Method::DELETE,
        ])
        .allow_headers([
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            ACCESS_CONTROL_ALLOW_HEADERS,
            ACCESS_CONTROL_REQUEST_HEADERS,
            ACCESS_CONTROL_REQUEST_METHOD,
            CONTENT_TYPE,
        ])
        .allow_origin(
            std::env::var(env_values::WEB_CLIENT_URL)
                .unwrap()
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_credentials(true);

    let app = Router::new()
        .route("/", get(routers::root::get_hello_world::handler))
        .route("/category", get(routers::category::get_categories::handler))
        .route(
            "/category/:category_id",
            get(routers::category::get_category::handler),
        )
        .route(
            "/category/:category_id/posts",
            get(routers::category::get_category_posts::handler),
        )
        .route(
            "/post/:post_id",
            get(routers::post::get_post_by_post_id::handler),
        )
        .route("/post", post(routers::post::create_post::handler))
        .route("/auth", put(routers::auth::get_auth_cookie::handler))
        .route("/auth", delete(routers::auth::remove_auth_cookie::handler))
        .route(
            "/writer/:writer_id",
            get(routers::writer::get_writer_by_writer_id::handler),
        )
        .route(
            "/writer/:writer_id/posts",
            get(routers::writer::get_posts_by_writer_id::handler),
        )
        .route(
            "/profile",
            get(routers::auth::get_writer_id_from_auth_cookie::handler),
        )
        .route("/profile", patch(routers::writer::update_writer::handler))
        .layer(cors)
        .with_state(state);

    // run as lambda
    // lambda_http::run(app).await

    // run as axum
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
