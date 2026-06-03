use crate::domain::ai_insight::entity::{AiInsight, InsightType, NewAiInsight};
use crate::error::AppError;
use serde::Deserialize;
use std::str::FromStr;
use worker::D1Database;

#[derive(Deserialize)]
struct AiInsightRow {
    id: String,
    workspace_id: String,
    insight_type: String,
    title: String,
    content: String,
    metadata: Option<String>,
    created_at: String,
}

impl From<AiInsightRow> for AiInsight {
    fn from(r: AiInsightRow) -> Self {
        Self {
            id: r.id,
            workspace_id: r.workspace_id,
            insight_type: InsightType::from_str(&r.insight_type).unwrap_or(InsightType::Tip),
            title: r.title,
            content: r.content,
            metadata: r.metadata.and_then(|s| serde_json::from_str(&s).ok()),
            created_at: r.created_at,
        }
    }
}

pub struct D1AiInsightRepository {
    db: D1Database,
}

impl D1AiInsightRepository {
    pub fn new(db: D1Database) -> Self { Self { db } }

    pub async fn create(&self, insight: NewAiInsight) -> Result<AiInsight, AppError> {
        let metadata_str = insight.metadata.map(|v| v.to_string());
        self.db
            .prepare("INSERT INTO ai_insights (id, workspace_id, insight_type, title, content, metadata, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7)")
            .bind(&[insight.id.clone().into(), insight.workspace_id.into(), insight.insight_type.to_string().into(), insight.title.into(), insight.content.into(), metadata_str.into(), insight.created_at.into()])
            .map_err(|e| AppError::Internal(e.to_string()))?
            .run().await.map_err(|e| AppError::Internal(e.to_string()))?;
        self.find_by_id(&insight.id).await?.ok_or_else(|| AppError::Internal("insert failed".into()))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<AiInsight>, AppError> {
        self.db.prepare("SELECT * FROM ai_insights WHERE id = ?1")
            .bind(&[id.into()]).map_err(|e| AppError::Internal(e.to_string()))?
            .first::<AiInsightRow>(None).await
            .map_err(|e| AppError::Internal(e.to_string())).map(|r| r.map(Into::into))
    }

    pub async fn list_by_workspace(&self, workspace_id: &str, insight_type: Option<&str>) -> Result<Vec<AiInsight>, AppError> {
        let results = self.db
            .prepare("SELECT * FROM ai_insights WHERE workspace_id = ?1 AND (?2 IS NULL OR insight_type = ?2) ORDER BY created_at DESC")
            .bind(&[workspace_id.into(), insight_type.map(String::from).into()])
            .map_err(|e| AppError::Internal(e.to_string()))?
            .all().await.map_err(|e| AppError::Internal(e.to_string()))?;
        results.results::<AiInsightRow>()
            .map_err(|e| AppError::Internal(e.to_string()))
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        self.db.prepare("DELETE FROM ai_insights WHERE id = ?1")
            .bind(&[id.into()]).map_err(|e| AppError::Internal(e.to_string()))?
            .run().await.map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
