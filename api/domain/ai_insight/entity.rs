use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiInsight {
    pub id: String,
    pub workspace_id: String,
    pub insight_type: InsightType,
    pub title: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
}

#[derive(Debug)]
pub struct NewAiInsight {
    pub id: String,
    pub workspace_id: String,
    pub insight_type: InsightType,
    pub title: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InsightType {
    Tip,
    Reduction,
    Recommendation,
    Analysis,
}

impl std::fmt::Display for InsightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tip => write!(f, "tip"),
            Self::Reduction => write!(f, "reduction"),
            Self::Recommendation => write!(f, "recommendation"),
            Self::Analysis => write!(f, "analysis"),
        }
    }
}

impl std::str::FromStr for InsightType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tip" => Ok(Self::Tip),
            "reduction" => Ok(Self::Reduction),
            "recommendation" => Ok(Self::Recommendation),
            "analysis" => Ok(Self::Analysis),
            _ => Err(format!("unknown insight type: {s}")),
        }
    }
}
