use serde::{Deserialize, Serialize};
use zod_rs::prelude::*;

#[derive(Deserialize, ZodSchema)]
pub struct CreateTransactionRequest {
    #[zod(min(0.01), max(1_000_000_000_000.0))]
    pub amount: f64,
    pub currency: Option<String>,
    #[zod(regex(r"^(Food|Transport|Shopping|Entertainment|Health|Education|Utilities|Salary|Investment|Other)$"))]
    pub category: String,
    pub description: Option<String>,
    #[zod(regex(r"^\d{4}-\d{2}-\d{2}$"))]
    pub transaction_date: String,
    #[zod(regex(r"^(income|expense)$"))]
    pub transaction_type: String,
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    fn valid_transaction() -> serde_json::Value {
        serde_json::json!({
            "amount": 100.0,
            "category": "Food",
            "transaction_date": "2024-01-15",
            "transaction_type": "expense"
        })
    }

    #[wasm_bindgen_test]
    fn create_transaction_valid() {
        assert!(CreateTransactionRequest::validate_and_parse(&valid_transaction()).is_ok());
    }

    #[wasm_bindgen_test]
    fn create_transaction_amount_below_min() {
        let v = serde_json::json!({
            "amount": 0.001,
            "category": "Food",
            "transaction_date": "2024-01-15",
            "transaction_type": "expense"
        });
        assert!(CreateTransactionRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn create_transaction_invalid_category() {
        let v = serde_json::json!({
            "amount": 50.0,
            "category": "InvalidCategory",
            "transaction_date": "2024-01-15",
            "transaction_type": "expense"
        });
        assert!(CreateTransactionRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn create_transaction_invalid_date_format() {
        let v = serde_json::json!({
            "amount": 50.0,
            "category": "Food",
            "transaction_date": "15-01-2024",
            "transaction_type": "expense"
        });
        assert!(CreateTransactionRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn create_transaction_invalid_type() {
        let v = serde_json::json!({
            "amount": 50.0,
            "category": "Food",
            "transaction_date": "2024-01-15",
            "transaction_type": "transfer"
        });
        assert!(CreateTransactionRequest::validate_and_parse(&v).is_err());
    }
}
