use crate::domain::bank_statement::entity::{BankStatement, NewBankStatement, ProcessingStatus};
use crate::error::AppError;

pub trait BankStatementRepository {
    async fn create(&self, stmt: NewBankStatement) -> Result<BankStatement, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<BankStatement>, AppError>;
    async fn list_by_workspace(&self, workspace_id: &str) -> Result<Vec<BankStatement>, AppError>;
    async fn update_status(
        &self,
        id: &str,
        status: ProcessingStatus,
        parsed: Option<serde_json::Value>,
        summary: Option<String>,
        updated_at: &str,
    ) -> Result<BankStatement, AppError>;
}
