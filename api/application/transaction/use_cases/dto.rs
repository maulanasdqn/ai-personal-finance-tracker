use crate::domain::transaction::entity::{TransactionCategory, TransactionType};

pub struct CreateTransactionInput {
    pub workspace_id: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub category: TransactionCategory,
    pub description: Option<String>,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
    pub created_by: String,
}
