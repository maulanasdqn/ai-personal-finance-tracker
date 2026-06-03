use crate::domain::transaction::entity::{NewTransaction, Transaction, TransactionPatch};
use crate::error::AppError;
use std::future::Future;

pub struct TransactionFilter {
    pub workspace_id: String,
    pub category: Option<String>,
    pub transaction_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub trait TransactionRepository: Send + Sync {
    fn create(&self, tx: NewTransaction) -> impl Future<Output = Result<Transaction, AppError>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<Option<Transaction>, AppError>> + Send;
    fn list(&self, filter: TransactionFilter) -> impl Future<Output = Result<Vec<Transaction>, AppError>> + Send;
    fn update(&self, id: &str, patch: TransactionPatch) -> impl Future<Output = Result<Transaction, AppError>> + Send;
    fn delete(&self, id: &str) -> impl Future<Output = Result<(), AppError>> + Send;
    fn summary_by_category(&self, workspace_id: &str, date_from: &str, date_to: &str) -> impl Future<Output = Result<Vec<CategorySummary>, AppError>> + Send;
}

#[derive(Debug)]
pub struct CategorySummary {
    pub category: String,
    pub total: f64,
    pub count: i64,
    pub transaction_type: String,
}
