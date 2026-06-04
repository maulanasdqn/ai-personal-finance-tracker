use crate::domain::transaction::entity::{TransactionCategory, TransactionType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub amount: f64,
    pub currency: Option<String>,
    pub category: TransactionCategory,
    pub description: Option<String>,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
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
            id: t.id,
            workspace_id: t.workspace_id,
            amount: t.amount,
            currency: t.currency,
            category: t.category.to_string(),
            description: t.description,
            transaction_date: t.transaction_date,
            transaction_type: t.transaction_type.to_string(),
            source: t.source.to_string(),
            created_by: t.created_by,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    fn parse(v: serde_json::Value) -> Result<CreateTransactionRequest, serde_json::Error> {
        serde_json::from_value(v)
    }

    #[wasm_bindgen_test]
    fn create_transaction_valid() {
        assert!(parse(serde_json::json!({
            "amount": 100.0, "category": "Food",
            "transaction_date": "2024-01-15", "transaction_type": "expense"
        }))
        .is_ok());
    }

    #[wasm_bindgen_test]
    fn create_transaction_invalid_category() {
        assert!(parse(serde_json::json!({
            "amount": 50.0, "category": "InvalidCategory",
            "transaction_date": "2024-01-15", "transaction_type": "expense"
        }))
        .is_err());
    }

    #[wasm_bindgen_test]
    fn create_transaction_invalid_type() {
        assert!(parse(serde_json::json!({
            "amount": 50.0, "category": "Food",
            "transaction_date": "2024-01-15", "transaction_type": "transfer"
        }))
        .is_err());
    }
}
