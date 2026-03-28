use crate::middleware::auth::Claims;
use crate::models::account::{Account, CreateAccountDto, DepositDto, TransferDto};
use crate::models::transaction::{Transaction, TransactionType};
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

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao iniciar transação",
            )
                .into_response();
        }
    };

    let update_result = sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2 RETURNING balance",
        payload.amount,
        account_id
    )
    .fetch_one(&mut *tx)
    .await;

    let balance = match update_result {
        Ok(record) => record.balance,
        Err(_) => return (StatusCode::NOT_FOUND, "Conta não encontrada").into_response(),
    };

    let history_result = sqlx::query!(
        r#"
        INSERT INTO transactions (account_from_id, account_to_id, amount, transaction_type)
        VALUES ($1, $2, $3, $4)
        "#,
        None as Option<Uuid>,
        account_id,
        payload.amount,
        TransactionType::Deposit as _
    )
    .execute(&mut *tx)
    .await;

    if history_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao gravar histórico",
        )
            .into_response();
    }

    match tx.commit().await {
        Ok(_) => (StatusCode::OK, Json(balance)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao confirmar depósito",
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn transfer(
    claims: Claims,
    State(pool): State<PgPool>,
    Json(payload): Json<TransferDto>,
) -> impl IntoResponse {
    let account_owner = sqlx::query!(
        "SELECT user_id FROM accounts WHERE id = $1",
        payload.from_account_id
    )
    .fetch_optional(&pool)
    .await;

    match account_owner {
        Ok(Some(record)) if record.user_id.to_string() == claims.sub => {}
        _ => {
            return (
                StatusCode::FORBIDDEN,
                "Você não tem permissão para transferir desta conta",
            )
                .into_response();
        }
    }

    if payload.amount <= rust_decimal::Decimal::ZERO {
        return (StatusCode::BAD_REQUEST, "O valor deve ser maior que zero").into_response();
    }

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao iniciar transação",
            )
                .into_response();
        }
    };

    let debit_result = sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2 AND balance >= $1 RETURNING balance",
        payload.amount,
        payload.from_account_id
    )
    .fetch_optional(&mut *tx)
    .await;

    if let Ok(None) = debit_result {
        return (StatusCode::BAD_REQUEST, "Saldo insuficiente").into_response();
    }

    if (sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        payload.amount,
        payload.to_account_id
    )
    .execute(&mut *tx)
    .await)
        .is_err()
    {
        return (StatusCode::BAD_REQUEST, "Conta de destino não encontrada").into_response();
    }

    let history_result = sqlx::query!(
        r#"
        INSERT INTO transactions (account_from_id, account_to_id, amount, transaction_type)
        VALUES ($1, $2, $3, $4)
        "#,
        payload.from_account_id,
        payload.to_account_id,
        payload.amount,
        TransactionType::Transfer as _
    )
    .execute(&mut *tx)
    .await;

    if history_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao gravar histórico",
        )
            .into_response();
    }

    match tx.commit().await {
        Ok(_) => (StatusCode::OK, "Transferência e registro concluídos").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao confirmar operação",
        )
            .into_response(),
    }
}

pub async fn get_statement(
    claims: crate::middleware::auth::Claims,
    State(pool): State<PgPool>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    let account_check = sqlx::query!("SELECT user_id FROM accounts WHERE id = $1", account_id)
        .fetch_optional(&pool)
        .await;

    match account_check {
        Ok(Some(record)) => {
            if record.user_id.to_string() != claims.sub {
                return (
                    StatusCode::FORBIDDEN,
                    "Você não tem permissão para ver este extrato",
                )
                    .into_response();
            }
        }
        Ok(None) => return (StatusCode::NOT_FOUND, "Conta não encontrada").into_response(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao validar conta").into_response();
        }
    }

    let transactions = sqlx::query_as!(
        Transaction,
        r#"
        SELECT id, account_from_id, account_to_id, amount, transaction_type as "transaction_type: _", created_at 
        FROM transactions 
        WHERE account_from_id = $1 OR account_to_id = $1
        ORDER BY created_at DESC
        "#,
        account_id
    )
    .fetch_all(&pool)
    .await;

    match transactions {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(e) => {
            eprintln!("Erro ao buscar extrato: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao gerar extrato").into_response()
        }
    }
}
