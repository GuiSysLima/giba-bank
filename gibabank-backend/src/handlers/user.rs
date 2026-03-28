use crate::models::user::{AuthResponse, CreateUserDto, LoginDto, User};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserDto>,
) -> impl IntoResponse {
    let hashed_password = match hash(payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao processar senha").into_response();
        }
    };

    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (full_name, cpf_cnpj, email, password_hash, user_type)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, full_name, cpf_cnpj, email, password_hash, user_type as "user_type: _", created_at
        "#,
        payload.full_name,
        payload.cpf_cnpj,
        payload.email,
        hashed_password,
        payload.user_type as _
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            if let Some(db_error) = e.as_database_error()
                && db_error.code() == Some(std::borrow::Cow::Borrowed("23505"))
            {
                return (
                    StatusCode::CONFLICT,
                    "Usuário com este CPF ou Email já existe",
                )
                    .into_response();
            }
            eprintln!("Erro ao criar usuário: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro interno no servidor",
            )
                .into_response()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginDto>) -> impl IntoResponse {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, full_name, cpf_cnpj, email, password_hash, user_type as "user_type: _", created_at FROM users WHERE email = $1"#,
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        _ => return (StatusCode::UNAUTHORIZED, "Credenciais inválidas").into_response(),
    };

    let is_valid = bcrypt::verify(payload.password, &user.password_hash).unwrap_or(false);
    if !is_valid {
        return (StatusCode::UNAUTHORIZED, "Credenciais inválidas").into_response();
    }

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret_do_giba_bank".as_ref()),
    )
    .unwrap();

    (StatusCode::OK, Json(AuthResponse { token, user })).into_response()
}
