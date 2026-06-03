use crate::domain::bank_statement::entity::{BankStatement, FileType, NewBankStatement, ProcessingStatus};
use crate::error::AppError;
use serde::Deserialize;
use std::str::FromStr;
use worker::D1Database;

#[derive(Deserialize)]
struct BankStatementRow {
    id: String,
    workspace_id: String,
    file_key: String,
    file_name: String,
    file_type: String,
    status: String,
    parsed_transactions: Option<String>,
    ai_summary: Option<String>,
    created_by: String,
    created_at: String,
    updated_at: String,
}

impl From<BankStatementRow> for BankStatement {
    fn from(r: BankStatementRow) -> Self {
        Self {
            id: r.id,
            workspace_id: r.workspace_id,
            file_key: r.file_key,
            file_name: r.file_name,
            file_type: FileType::from_str(&r.file_type).unwrap_or(FileType::Image),
            status: ProcessingStatus::from_str(&r.status).unwrap_or(ProcessingStatus::Pending),
            parsed_transactions: r.parsed_transactions.and_then(|s| serde_json::from_str(&s).ok()),
            ai_summary: r.ai_summary,
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl std::str::FromStr for ProcessingStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "processed" => Ok(Self::Processed),
            "failed" => Ok(Self::Failed),
            _ => Err(format!("unknown status: {}", s)),
        }
    }
}

pub struct D1BankStatementRepository {
    db: D1Database,
}

impl D1BankStatementRepository {
    pub fn new(db: D1Database) -> Self { Self { db } }

    pub async fn create(&self, s: NewBankStatement) -> Result<BankStatement, AppError> {
        self.db
            .prepare("INSERT INTO bank_statements (id, workspace_id, file_key, file_name, file_type, status, created_by, created_at, updated_at) VALUES (?1,?2,?3,?4,?5,'pending',?6,?7,?8)")
            .bind(&[s.id.clone().into(), s.workspace_id.into(), s.file_key.into(), s.file_name.into(), s.file_type.to_string().into(), s.created_by.into(), s.created_at.into(), s.updated_at.into()])
            .map_err(|_| AppError::Internal)?
            .run().await.map_err(|_| AppError::Internal)?;
        self.find_by_id(&s.id).await?.ok_or_else(|| AppError::Internal)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<BankStatement>, AppError> {
        self.db.prepare("SELECT * FROM bank_statements WHERE id = ?1")
            .bind(&[id.into()]).map_err(|_| AppError::Internal)?
            .first::<BankStatementRow>(None).await
            .map_err(|_| AppError::Internal).map(|r| r.map(Into::into))
    }

    pub async fn list_by_workspace(&self, workspace_id: &str) -> Result<Vec<BankStatement>, AppError> {
        let results = self.db.prepare("SELECT * FROM bank_statements WHERE workspace_id = ?1 ORDER BY created_at DESC")
            .bind(&[workspace_id.into()]).map_err(|_| AppError::Internal)?
            .all().await.map_err(|_| AppError::Internal)?;
        results.results::<BankStatementRow>()
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn update_status(&self, id: &str, status: ProcessingStatus, parsed: Option<serde_json::Value>, summary: Option<String>, now: &str) -> Result<BankStatement, AppError> {
        let parsed_str = parsed.map(|v| v.to_string());
        self.db.prepare("UPDATE bank_statements SET status = ?1, parsed_transactions = ?2, ai_summary = ?3, updated_at = ?4 WHERE id = ?5")
            .bind(&[status.to_string().into(), parsed_str.into(), summary.into(), now.into(), id.into()])
            .map_err(|_| AppError::Internal)?
            .run().await.map_err(|_| AppError::Internal)?;
        self.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("statement not found".into()))
    }

}
