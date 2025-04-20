mod env_values;
mod extractors;
mod repositories;
mod routers;

use axum::{
    extract::FromRef,
    http::HeaderValue,
    routing::{delete, get, patch, post},
    Router,
};
use http::{
    header::{
        ACCEPT_ENCODING, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD, CONTENT_TYPE,
    },
    Method,
};
use lambda_http::tower::ServiceBuilder;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    decompression::RequestDecompressionLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[derive(Clone, FromRef)]
pub struct AppState {
    pg_pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    let db_connection_str = std::env::var(env_values::DATABASE_URL).unwrap();

    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let state = AppState { pg_pool };

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
            ACCEPT_ENCODING,
        ])
        .allow_origin(
            std::env::var(env_values::WEB_CLIENT_URL)
                .unwrap()
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_credentials(true);

    let category_router = Router::new()
        .route("/category", get(routers::category::get_categories::handler))
        .route(
            "/category/{category_id}",
            get(routers::category::get_category::handler),
        )
        .route(
            "/category/{category_id}/posts",
            get(routers::category::get_category_posts::handler),
        );

    let post_attachment_router = Router::new()
        .route(
            "/post-attachment",
            post(routers::post_attachment::create_post_attachment::handler),
        )
        .route(
            "/post-attachment/{attachment_id}",
            get(routers::post_attachment::get_post_attachment_by_attachment_id::handler),
        );

    let post_router = Router::new()
        .route(
            "/post/{post_id}",
            get(routers::post::get_post_by_post_id::handler),
        )
        .route(
            "/post/{post_id}",
            patch(routers::post::update_post::handler),
        )
        .route(
            "/post/{post_id}",
            delete(routers::post::delete_post::handler),
        )
        .route("/post", post(routers::post::create_post::handler));

    let writer_router = Router::new()
        .route(
            "/writer/{writer_id}",
            get(routers::writer::get_writer_by_writer_id::handler),
        )
        // .route("/writer", post(routers::auth::create_writer::handler))
        .route(
            "/writer/{writer_id}/posts",
            get(routers::writer::get_posts_by_writer_id::handler),
        );

    let profile_router = Router::new()
        .route(
            "/profile",
            get(routers::auth::get_writer_id_from_auth_header::handler),
        )
        .route("/profile", patch(routers::writer::update_writer::handler))
        .route(
            "/profile/posts",
            get(routers::writer::get_posts_by_authed_writer::handler),
        );

    let auth_router = Router::new().route(
        "/auth/access-token",
        post(routers::auth::get_access_token::handler),
    );

    let app = Router::new()
        .route("/", get(routers::root::get_hello_world::handler))
        .merge(category_router)
        .merge(post_router)
        .merge(post_attachment_router)
        .merge(writer_router)
        .merge(profile_router)
        .merge(auth_router)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .on_response(DefaultOnResponse::new().level(Level::DEBUG)),
                )
                .layer(cors)
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .with_state(state);

    // run as lambda
    lambda_http::run(app).await

    // run as axum
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
