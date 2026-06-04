use serde::{Deserialize, Serialize};
use zod_rs::prelude::*;

#[derive(Deserialize, ZodSchema)]
pub struct GenerateInsightsRequest {
    #[zod(regex(r"^\d{4}-\d{2}-\d{2}$"))]
    pub date_from: String,
    #[zod(regex(r"^\d{4}-\d{2}-\d{2}$"))]
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
        Self {
            id: i.id,
            workspace_id: i.workspace_id,
            insight_type: i.insight_type.to_string(),
            title: i.title,
            content: i.content,
            metadata: i.metadata,
            created_at: i.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    #[wasm_bindgen_test]
    fn generate_insights_valid_dates() {
        let v = serde_json::json!({
            "date_from": "2024-01-01",
            "date_to": "2024-12-31"
        });
        assert!(GenerateInsightsRequest::validate_and_parse(&v).is_ok());
    }

    #[wasm_bindgen_test]
    fn generate_insights_invalid_date_from() {
        let v = serde_json::json!({
            "date_from": "01-01-2024",
            "date_to": "2024-12-31"
        });
        assert!(GenerateInsightsRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn generate_insights_invalid_date_to() {
        let v = serde_json::json!({
            "date_from": "2024-01-01",
            "date_to": "31/12/2024"
        });
        assert!(GenerateInsightsRequest::validate_and_parse(&v).is_err());
    }
}
