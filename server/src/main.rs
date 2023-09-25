mod extractors;
mod repositories;
mod routers;

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

// our application state
#[derive(Clone)]
struct AppState {
    // that holds the key used to sign cookies
    key: Key,
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    let state = AppState {
        // key: Key::from("my secret key".as_bytes()),
        key: Key::generate(),
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(routers::root::get_hello_world::handler))
        .route("/category", get(routers::category::get_categories::handler))
        .route(
            "/category/:category_id/posts",
            get(routers::category::get_category_posts::handler),
        )
        .route(
            "/post/:post_id",
            get(routers::post::get_post_by_post_id::handler),
        )
        .route("/auth", post(routers::auth::get_auth_cookie::handler))
        .with_state(pool)
        .with_state(state);

    lambda_http::run(app).await
}
