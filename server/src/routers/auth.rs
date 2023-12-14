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

pub mod get_access_token_with_google_oauth {
    use axum::{
        extract::Query,
        response::{IntoResponse, Redirect},
    };
    use chrono::Utc;
    use http::{HeaderMap, HeaderValue, StatusCode};
    use jsonwebtoken::{encode, EncodingKey};
    use serde::{Deserialize, Serialize};

    use crate::{env_values, extractors::DatabaseConnection};

    #[derive(Deserialize)]
    pub struct AuthCode {
        code: String,
    }

    #[derive(Deserialize)]
    struct GoogleAccessToken {
        access_token: String,
    }

    async fn get_access_token_from_google_api(
        auth_code: String,
    ) -> Result<GoogleAccessToken, reqwest::Error> {
        let client = reqwest::Client::new();

        let res = client
            .post("https://oauth2.googleapis.com/token")
            .json(&[
                ("code", auth_code),
                (
                    "client_id",
                    std::env::var(env_values::GOOGLE_CLIENT_ID).unwrap(),
                ),
                (
                    "client_secret",
                    std::env::var(env_values::GOOGLE_CLIENT_SECRET).unwrap(),
                ),
                (
                    "redirect_uri",
                    std::env::var(env_values::GOOGLE_REDIRECT_URI).unwrap(),
                ),
                ("grant_type", "authorization_code".into()),
            ])
            .send()
            .await?
            .json::<GoogleAccessToken>()
            .await?;

        Ok(res)
    }

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

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: u64,
    }

    struct WriterRow {
        id: i64,
    }

    pub async fn handler(
        auth_code_payload: Query<AuthCode>,
        DatabaseConnection(mut conn): DatabaseConnection,
    ) -> Result<impl IntoResponse, StatusCode> {
        let access_token =
            match get_access_token_from_google_api(auth_code_payload.code.clone()).await {
                Ok(access_token) => access_token,
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            };

        let google_user = match get_google_user_from_google_api(access_token.access_token).await {
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

        let mut headers = HeaderMap::new();
        headers.insert("access-token", HeaderValue::from(token.unwrap()));

        Ok(Redirect::new(
            std::env::var(env_values::GOOGLE_REDIRECT_URI).unwrap(),
        ))
    }
}
