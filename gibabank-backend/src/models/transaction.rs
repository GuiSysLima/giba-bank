use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "UPPERCASE")]
pub enum TransactionType {
    Deposit,
    Transfer,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub account_from_id: Option<Uuid>,
    pub account_to_id: Uuid,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
    pub created_at: Option<DateTime<Utc>>,
}
