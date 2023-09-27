pub mod get_auth_cookie {

    use axum::{extract::Json, http::StatusCode, response::IntoResponse};
    use axum_extra::extract::{cookie::Cookie, SignedCookieJar};
    use cookie::time::{Duration, OffsetDateTime};

    use crate::extractors::DatabaseConnection;

    #[derive(serde::Deserialize)]
    pub struct AuthBody {
        email: String,
    }

    struct WriterRow {
        id: i64,
    }

    pub const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        jar: SignedCookieJar,
        Json(payload): Json<AuthBody>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let access_token_cookie = jar.get(ACCESS_TOKEN_COOKIE_NAME);

        if access_token_cookie.is_some() {
            return Ok((jar, StatusCode::OK));
        }

        let result = sqlx::query_as!(
            WriterRow,
            "select id from writer where email = $1",
            payload.email
        )
        .fetch_one(&mut *conn)
        .await;

        if result.is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        };

        let writer = result.unwrap();

        let mut now = OffsetDateTime::now_utc();
        now += Duration::weeks(52);

        let mut auth_cookie = Cookie::new(ACCESS_TOKEN_COOKIE_NAME, writer.id.to_string());

        auth_cookie.set_expires(now);

        let jar = jar.add(auth_cookie);

        Ok((jar, StatusCode::OK))
    }
}
