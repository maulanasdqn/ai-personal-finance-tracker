use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankStatement {
    pub id: String,
    pub workspace_id: String,
    pub file_key: String,
    pub file_name: String,
    pub file_type: FileType,
    pub status: ProcessingStatus,
    pub parsed_transactions: Option<serde_json::Value>,
    pub ai_summary: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct NewBankStatement {
    pub id: String,
    pub workspace_id: String,
    pub file_key: String,
    pub file_name: String,
    pub file_type: FileType,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Pdf,
    Image,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pdf => write!(f, "pdf"),
            Self::Image => write!(f, "image"),
        }
    }
}

impl std::str::FromStr for FileType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pdf" => Ok(Self::Pdf),
            "image" => Ok(Self::Image),
            _ => Err(format!("unknown file type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Processed,
    Failed,
}

impl std::fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::Processed => write!(f, "processed"),
            Self::Failed => write!(f, "failed"),
        }
    }
}
