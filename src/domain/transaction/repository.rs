use crate::domain::transaction::entity::{NewTransaction, Transaction};
use crate::error::AppError;

pub struct TransactionFilter {
    pub workspace_id: String,
    pub category: Option<String>,
    pub transaction_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub trait TransactionRepository {
    async fn create(&self, tx: NewTransaction) -> Result<Transaction, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Transaction>, AppError>;
    async fn list(&self, filter: TransactionFilter) -> Result<Vec<Transaction>, AppError>;
    async fn delete(&self, id: &str) -> Result<(), AppError>;
}
