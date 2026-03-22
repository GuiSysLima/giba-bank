use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_type", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum UserType {
    Common,
    Merchant,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub cpf_cnpj: String,
    pub email: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub password_hash: String,
    pub user_type: UserType,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub full_name: String,
    pub cpf_cnpj: String,
    pub email: String,
    pub password: String,
    pub user_type: UserType,
}
