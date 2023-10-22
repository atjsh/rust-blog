pub mod get_writer_id_from_auth_cookie {
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
            &EncodingKey::from_secret(std::env::var(env_values::COOKIE_SECRET).unwrap().as_bytes()),
        );

        Ok(token.unwrap())
    }
}
