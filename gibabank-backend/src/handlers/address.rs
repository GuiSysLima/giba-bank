use crate::models::address::{Address, CreateAddressDto};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

pub async fn create_address(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateAddressDto>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Address,
        r#"
        INSERT INTO addresses (user_id, street, number, complement, neighborhood, city, state, zip_code)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, street, number, complement, neighborhood, city, state, zip_code, created_at
        "#,
        payload.user_id,
        payload.street,
        payload.number,
        payload.complement,
        payload.neighborhood,
        payload.city,
        payload.state,
        payload.zip_code
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(address) => (StatusCode::CREATED, Json(address)).into_response(),
        Err(e) => {
            eprintln!("Erro ao cadastrar endereço: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao cadastrar endereço",
            )
                .into_response()
        }
    }
}
