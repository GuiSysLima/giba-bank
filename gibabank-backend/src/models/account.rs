use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
    Checking,
    Savings,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_number: String,
    pub agency: String,
    pub balance: Decimal,
    pub account_type: AccountType,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateAccountDto {
    pub user_id: Uuid,
    pub account_number: String,
    pub agency: String,
    pub account_type: AccountType,
}
