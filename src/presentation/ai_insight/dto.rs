use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GenerateInsightsRequest {
    pub date_from: String,
    pub date_to: String,
}

#[derive(Serialize)]
pub struct AiInsightResponse {
    pub id: String,
    pub workspace_id: String,
    pub insight_type: String,
    pub title: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
}

impl From<crate::domain::ai_insight::entity::AiInsight> for AiInsightResponse {
    fn from(i: crate::domain::ai_insight::entity::AiInsight) -> Self {
        Self { id: i.id, workspace_id: i.workspace_id, insight_type: i.insight_type.to_string(), title: i.title, content: i.content, metadata: i.metadata, created_at: i.created_at }
    }
}
