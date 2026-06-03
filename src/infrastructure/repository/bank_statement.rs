use crate::domain::bank_statement::entity::{BankStatement, FileType, NewBankStatement, ProcessingStatus};
use crate::domain::bank_statement::repository::BankStatementRepository;
use crate::error::AppError;
use rust_d1_orm::{opt_js, D1Model, Order, Query, Set, Table};
use serde::Deserialize;
use std::str::FromStr;
use worker::{wasm_bindgen::JsValue, D1Database};

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

impl D1Model for BankStatementRow {
    const TABLE: &'static str = "bank_statements";
    const COLUMNS: &'static [&'static str] = &[
        "id", "workspace_id", "file_key", "file_name", "file_type",
        "status", "parsed_transactions", "ai_summary",
        "created_by", "created_at", "updated_at",
    ];
    fn values(&self) -> Vec<JsValue> {
        vec![
            self.id.clone().into(), self.workspace_id.clone().into(),
            self.file_key.clone().into(), self.file_name.clone().into(),
            self.file_type.clone().into(), self.status.clone().into(),
            opt_js(self.parsed_transactions.clone()), opt_js(self.ai_summary.clone()),
            self.created_by.clone().into(), self.created_at.clone().into(),
            self.updated_at.clone().into(),
        ]
    }
}

impl From<BankStatementRow> for BankStatement {
    fn from(r: BankStatementRow) -> Self {
        Self {
            id: r.id, workspace_id: r.workspace_id, file_key: r.file_key,
            file_name: r.file_name,
            file_type: FileType::from_str(&r.file_type).unwrap_or(FileType::Image),
            status: ProcessingStatus::from_str(&r.status).unwrap_or(ProcessingStatus::Pending),
            parsed_transactions: r.parsed_transactions.and_then(|s| serde_json::from_str(&s).ok()),
            ai_summary: r.ai_summary, created_by: r.created_by,
            created_at: r.created_at, updated_at: r.updated_at,
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

pub struct D1BankStatementRepository { db: D1Database }

impl D1BankStatementRepository {
    pub fn new(db: D1Database) -> Self { Self { db } }
}

impl BankStatementRepository for D1BankStatementRepository {
    async fn create(&self, s: NewBankStatement) -> Result<BankStatement, AppError> {
        let row = BankStatementRow {
            id: s.id, workspace_id: s.workspace_id, file_key: s.file_key,
            file_name: s.file_name, file_type: s.file_type.to_string(),
            status: "pending".to_string(), parsed_transactions: None,
            ai_summary: None, created_by: s.created_by,
            created_at: s.created_at, updated_at: s.updated_at,
        };
        Table::<BankStatementRow>::new(&self.db)
            .insert(&row).await
            .map_err(|_| AppError::Internal)
            .map(Into::into)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<BankStatement>, AppError> {
        Table::<BankStatementRow>::new(&self.db)
            .find_one(Query::new().eq("id", id)).await
            .map_err(|_| AppError::Internal)
            .map(|r| r.map(Into::into))
    }

    async fn list_by_workspace(&self, workspace_id: &str) -> Result<Vec<BankStatement>, AppError> {
        Table::<BankStatementRow>::new(&self.db)
            .find_all(Query::new().eq("workspace_id", workspace_id).order_by("created_at", Order::Desc)).await
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    async fn update_status(&self, id: &str, status: ProcessingStatus, parsed: Option<serde_json::Value>, summary: Option<String>, now: &str) -> Result<BankStatement, AppError> {
        let parsed_str = parsed.map(|v| v.to_string());
        let set = Set::new()
            .field("status", status.to_string())
            .nullable_field("parsed_transactions", parsed_str)
            .nullable_field("ai_summary", summary)
            .field("updated_at", now);
        Table::<BankStatementRow>::new(&self.db)
            .update(set, Query::new().eq("id", id)).await
            .map_err(|_| AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("statement not found".into()))
            .map(Into::into)
    }
}
