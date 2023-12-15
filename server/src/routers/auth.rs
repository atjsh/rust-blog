pub mod get_writer_id_from_auth_header {
    use axum::{
        headers::{authorization::Bearer, Authorization},
        http::StatusCode,
        response::IntoResponse,
        TypedHeader,
    };
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use serde::{Deserialize, Serialize};

    use crate::env_values;

    fn unauthorized() -> http::StatusCode {
        StatusCode::UNAUTHORIZED
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: u64,
    }

    pub async fn handler(
        TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>,
    ) -> Result<impl IntoResponse, StatusCode> {
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

        Ok(writer_id.to_string())
    }
}

pub mod get_access_token {
    use axum::{response::IntoResponse, Json};
    use chrono::Utc;
    use http::StatusCode;
    use jsonwebtoken::{encode, EncodingKey};
    use serde::{Deserialize, Serialize};

    use crate::{env_values, extractors::DatabaseConnection};

    #[derive(serde::Deserialize)]
    pub struct AuthBody {
        email: String,
    }

    struct WriterRow {
        id: i64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: u64,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        Json(payload): Json<AuthBody>,
    ) -> Result<impl IntoResponse, StatusCode> {
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

        let mut expire_date = Utc::now();
        expire_date += chrono::Duration::days(365);

        let claims = Claims {
            sub: writer.id.to_string(),
            exp: expire_date.timestamp() as u64,
        };
        let token = encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(std::env::var(env_values::JWT_SECRET).unwrap().as_bytes()),
        );

        Ok(token.unwrap())
    }
}

pub mod get_access_token_by_google_access_token {
    use axum::{
        extract::Query,
        response::{IntoResponse, Response},
        Json,
    };
    use axum_extra::extract::SignedCookieJar;
    use chrono::Utc;
    use cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    };
    use http::StatusCode;
    use jsonwebtoken::{encode, EncodingKey};
    use serde::{Deserialize, Serialize};

    use crate::{env_values, extractors::DatabaseConnection};

    pub const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";

    #[derive(Deserialize)]
    struct GoogleUser {
        email: String,
        verified_email: bool,
    }
    async fn get_google_user_from_google_api(
        access_token: String,
    ) -> Result<GoogleUser, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
                access_token
            ))
            .send()
            .await?
            .json::<GoogleUser>()
            .await?;
        Ok(res)
    }

    #[derive(serde::Deserialize)]
    pub struct AuthBody {
        google_access_token: String,
    }

    #[derive(serde::Deserialize)]
    pub struct AuthQuery {
        token_location: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: u64,
    }
    struct WriterRow {
        id: i64,
    }

    pub async fn handler(
        DatabaseConnection(mut conn): DatabaseConnection,
        jar: SignedCookieJar,
        Json(body): Json<AuthBody>,
        query: Query<AuthQuery>,
    ) -> Result<Response, StatusCode> {
        let google_user = match get_google_user_from_google_api(body.google_access_token).await {
            Ok(google_user) => google_user,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        };

        if !google_user.verified_email {
            return Err(StatusCode::UNAUTHORIZED);
        };

        let result = sqlx::query_as!(
            WriterRow,
            "select id from writer where email = $1",
            google_user.email
        )
        .fetch_one(&mut *conn)
        .await;

        if result.is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        };

        let writer = result.unwrap();

        let mut expire_date = Utc::now();
        expire_date += chrono::Duration::days(365);

        let claims = Claims {
            sub: writer.id.to_string(),
            exp: expire_date.timestamp() as u64,
        };

        let token = encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(std::env::var(env_values::JWT_SECRET).unwrap().as_bytes()),
        );

        match query.token_location.as_str() {
            "cookie" => {
                let mut auth_cookie = Cookie::new(ACCESS_TOKEN_COOKIE_NAME, writer.id.to_string());

                let mut now = OffsetDateTime::now_utc();
                now += Duration::weeks(52);

                auth_cookie.set_expires(now);

                let jar = jar.add(auth_cookie);

                Ok((StatusCode::OK, jar).into_response())
            }
            "body" => Ok((StatusCode::OK, token.unwrap()).into_response()),
            _ => Err(StatusCode::BAD_REQUEST),
        }
    }
}
