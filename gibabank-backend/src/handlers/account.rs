use crate::models::account::{Account, CreateAccountDto, DepositDto};
use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[axum::debug_handler]
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
    .fetch_all(&pool)
    .await;

    match result {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(e) => {
            eprintln!("Erro ao inserir no banco: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar conta").into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn list_accounts_by_user(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Account,
        r#"
        SELECT id, user_id, account_number, agency, balance, 
               account_type as "account_type: _", 
               is_active as "is_active!", 
               created_at
        FROM accounts
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(accounts) => (StatusCode::OK, Json(accounts)).into_response(),
        Err(e) => {
            eprintln!("Erro ao buscar contas: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao buscar contas").into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn deposit(
    State(pool): State<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<DepositDto>,
) -> impl IntoResponse {
    if payload.amount <= rust_decimal::Decimal::ZERO {
        return (StatusCode::BAD_REQUEST, "O valor deve ser maior que zero").into_response();
    }

    let result = sqlx::query!(
        r#"
        UPDATE accounts 
        SET balance = balance + $1 
        WHERE id = $2
        RETURNING balance
        "#,
        payload.amount,
        account_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => (StatusCode::OK, Json(record.balance)).into_response(),
        Err(e) => {
            eprintln!("Erro ao realizar depósito: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao processar depósito",
            )
                .into_response()
        }
    }
}
