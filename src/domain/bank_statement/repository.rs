use crate::domain::bank_statement::entity::{BankStatement, NewBankStatement, ProcessingStatus};
use crate::error::AppError;
use std::future::Future;

pub trait BankStatementRepository: Send + Sync {
    fn create(&self, stmt: NewBankStatement) -> impl Future<Output = Result<BankStatement, AppError>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<Option<BankStatement>, AppError>> + Send;
    fn list_by_workspace(&self, workspace_id: &str) -> impl Future<Output = Result<Vec<BankStatement>, AppError>> + Send;
    fn update_status(&self, id: &str, status: ProcessingStatus, parsed_transactions: Option<serde_json::Value>, ai_summary: Option<String>) -> impl Future<Output = Result<BankStatement, AppError>> + Send;
    fn delete(&self, id: &str) -> impl Future<Output = Result<(), AppError>> + Send;
}
