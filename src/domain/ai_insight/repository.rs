use crate::domain::ai_insight::entity::{AiInsight, NewAiInsight};
use crate::error::AppError;
use std::future::Future;

pub trait AiInsightRepository: Send + Sync {
    fn create(&self, insight: NewAiInsight) -> impl Future<Output = Result<AiInsight, AppError>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<Option<AiInsight>, AppError>> + Send;
    fn list_by_workspace(&self, workspace_id: &str, insight_type: Option<&str>) -> impl Future<Output = Result<Vec<AiInsight>, AppError>> + Send;
    fn delete(&self, id: &str) -> impl Future<Output = Result<(), AppError>> + Send;
}
