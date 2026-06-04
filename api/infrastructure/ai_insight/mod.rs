use crate::domain::ai_insight::entity::{AiInsight, InsightType, NewAiInsight};
use crate::domain::ai_insight::repository::AiInsightRepository;
use crate::error::AppError;
use rust_d1_orm::{opt_js, D1Model, Order, Query, Table};
use serde::Deserialize;
use std::str::FromStr;
use worker::{wasm_bindgen::JsValue, D1Database};

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

impl D1Model for AiInsightRow {
    const TABLE: &'static str = "ai_insights";
    const COLUMNS: &'static [&'static str] = &[
        "id",
        "workspace_id",
        "insight_type",
        "title",
        "content",
        "metadata",
        "created_at",
    ];
    fn values(&self) -> Vec<JsValue> {
        vec![
            self.id.clone().into(),
            self.workspace_id.clone().into(),
            self.insight_type.clone().into(),
            self.title.clone().into(),
            self.content.clone().into(),
            opt_js(self.metadata.clone()),
            self.created_at.clone().into(),
        ]
    }
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
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }
}

impl AiInsightRepository for D1AiInsightRepository {
    async fn create_batch(&self, insights: Vec<NewAiInsight>) -> Result<Vec<AiInsight>, AppError> {
        let rows: Vec<AiInsightRow> = insights
            .into_iter()
            .map(|i| AiInsightRow {
                id: i.id,
                workspace_id: i.workspace_id,
                insight_type: i.insight_type.to_string(),
                title: i.title,
                content: i.content,
                metadata: i.metadata.map(|v| v.to_string()),
                created_at: i.created_at,
            })
            .collect();
        Table::<AiInsightRow>::new(&self.db)
            .insert_batch(&rows)
            .await
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    async fn was_recently_generated(
        &self,
        workspace_id: &str,
        cooldown_secs: i64,
    ) -> Result<bool, AppError> {
        let cutoff = (chrono::Utc::now() - chrono::Duration::seconds(cooldown_secs)).to_rfc3339();
        let count = Table::<AiInsightRow>::new(&self.db)
            .count(
                Query::new()
                    .eq("workspace_id", workspace_id)
                    .gte("created_at", cutoff),
            )
            .await
            .map_err(|_| AppError::Internal)?;
        Ok(count > 0)
    }

    async fn list_by_workspace(
        &self,
        workspace_id: &str,
        insight_type: Option<&str>,
    ) -> Result<Vec<AiInsight>, AppError> {
        let query = Query::new()
            .eq("workspace_id", workspace_id)
            .filter_optional("insight_type", insight_type)
            .order_by("created_at", Order::Desc);
        Table::<AiInsightRow>::new(&self.db)
            .find_all(query)
            .await
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }
}
