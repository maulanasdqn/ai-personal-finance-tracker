use crate::domain::ai_insight::entity::{AiInsight, NewAiInsight};
use crate::error::AppError;

pub trait AiInsightRepository {
    async fn create_batch(&self, insights: Vec<NewAiInsight>) -> Result<Vec<AiInsight>, AppError>;
    async fn was_recently_generated(&self, workspace_id: &str, cooldown_secs: i64) -> Result<bool, AppError>;
    async fn list_by_workspace(&self, workspace_id: &str, insight_type: Option<&str>) -> Result<Vec<AiInsight>, AppError>;
}
