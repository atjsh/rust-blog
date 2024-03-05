// pub mod create_writer {
//     use argon2::{
//         password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
//         Argon2,
//     };
//     use axum::{response::IntoResponse, Json};
//     use http::StatusCode;
//     use serde::Deserialize;

//     use crate::extractors::DatabaseConnection;

//     #[derive(Deserialize)]
//     pub struct Writer {
//         email: String,
//         password: String,
//     }

//     pub async fn handler(
//         DatabaseConnection(mut conn): DatabaseConnection,
//         Json(payload): Json<Writer>,
//     ) -> Result<impl IntoResponse, StatusCode> {
//         println!("email: {}", payload.email);
//         println!("password: {}", payload.password);

//         let salt: SaltString = SaltString::generate(&mut OsRng);

//         // Argon2 with default params (Argon2id v19)
//         let argon2 = Argon2::default();

//         let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt);

//         if password_hash.is_err() {
//             return Err(StatusCode::BAD_REQUEST);
//         }

//         let result = sqlx::query!(
//             "insert into writer (email, password_hashed, description) values ($1, $2, 'TEST') returning id",
//             payload.email,
//             password_hash.unwrap().to_string()
//         )
//         .fetch_one(&mut *conn)
//         .await;

//         if result.is_err() {
//             return Err(StatusCode::BAD_REQUEST);
//         }

//         Ok(StatusCode::CREATED)
//     }
// }

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
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    use axum::{response::IntoResponse, Json};
    use chrono::Utc;
    use http::StatusCode;
    use jsonwebtoken::{encode, EncodingKey};
    use serde::{Deserialize, Serialize};

    use crate::{env_values, extractors::DatabaseConnection};

    #[derive(serde::Deserialize)]
    pub struct AuthBody {
        email: String,
        password: String,
    }

    struct WriterRow {
        id: i64,
        password_hashed: String,
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
            "select id, password_hashed from writer where email = $1",
            payload.email
        )
        .fetch_one(&mut *conn)
        .await;

        if result.is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        };

        let writer = result.unwrap();

        let parsed_hash = PasswordHash::new(&writer.password_hashed);

        if parsed_hash.is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let password_passed = Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash.unwrap())
            .is_ok();

        if !password_passed {
            return Err(StatusCode::UNAUTHORIZED);
        }

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
