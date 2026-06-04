use serde::Serialize;

#[derive(Serialize)]
pub struct BankStatementResponse {
    pub id: String,
    pub workspace_id: String,
    pub file_name: String,
    pub file_type: String,
    pub status: String,
    pub parsed_transactions: Option<serde_json::Value>,
    pub ai_summary: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::domain::bank_statement::entity::BankStatement> for BankStatementResponse {
    fn from(s: crate::domain::bank_statement::entity::BankStatement) -> Self {
        Self {
            id: s.id,
            workspace_id: s.workspace_id,
            file_name: s.file_name,
            file_type: s.file_type.to_string(),
            status: s.status.to_string(),
            parsed_transactions: s.parsed_transactions,
            ai_summary: s.ai_summary,
            created_by: s.created_by,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}
