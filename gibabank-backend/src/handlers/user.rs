use crate::models::user::{CreateUserDto, User};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserDto>,
) -> impl IntoResponse {
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
        payload.password,
        payload.user_type as _
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            eprintln!("Erro ao criar usuário: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar usuário").into_response()
        }
    }
}
