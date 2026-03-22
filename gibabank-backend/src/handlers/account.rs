use crate::models::account::{Account, CreateAccountDto};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

pub async fn create_account(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateAccountDto>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Account,
        r#"
        INSERT INTO accounts (user_id, account_number, agency, account_type)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, account_number, agency, balance, 
                account_type as "account_type: _", 
                is_active as "is_active!", 
                created_at
        "#,
        payload.user_id,
        payload.account_number,
        payload.agency,
        payload.account_type as _
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(e) => {
            eprintln!("Erro ao inserir no banco: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar conta").into_response()
        }
    }
}
