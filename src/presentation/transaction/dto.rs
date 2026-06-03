use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub amount: f64,
    pub currency: Option<String>,
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: String,
    pub transaction_type: String,
}

#[derive(Deserialize)]
pub struct UpdateTransactionRequest {
    pub amount: Option<f64>,
    pub category: Option<String>,
    pub description: Option<Option<String>>,
    pub transaction_date: Option<String>,
    pub transaction_type: Option<String>,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub id: String,
    pub workspace_id: String,
    pub amount: f64,
    pub currency: String,
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: String,
    pub transaction_type: String,
    pub source: String,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::domain::transaction::entity::Transaction> for TransactionResponse {
    fn from(t: crate::domain::transaction::entity::Transaction) -> Self {
        Self {
            id: t.id, workspace_id: t.workspace_id, amount: t.amount, currency: t.currency,
            category: t.category, description: t.description, transaction_date: t.transaction_date,
            transaction_type: t.transaction_type.to_string(), source: t.source.to_string(),
            created_by: t.created_by, created_at: t.created_at, updated_at: t.updated_at,
        }
    }
}
