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

#[cfg(test)]
mod file_type_tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;
    use std::str::FromStr;

    #[wasm_bindgen_test]
    fn from_str_pdf() {
        assert_eq!(FileType::from_str("pdf").unwrap(), FileType::Pdf);
    }

    #[wasm_bindgen_test]
    fn from_str_image() {
        assert_eq!(FileType::from_str("image").unwrap(), FileType::Image);
    }

    #[wasm_bindgen_test]
    fn from_str_unknown_is_err() {
        assert!(FileType::from_str("video").is_err());
    }

    #[wasm_bindgen_test]
    fn to_string_pdf() {
        assert_eq!(FileType::Pdf.to_string(), "pdf");
    }

    #[wasm_bindgen_test]
    fn to_string_image() {
        assert_eq!(FileType::Image.to_string(), "image");
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

#[cfg(test)]
mod processing_status_tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    #[wasm_bindgen_test]
    fn pending_to_string() {
        assert_eq!(ProcessingStatus::Pending.to_string(), "pending");
    }

    #[wasm_bindgen_test]
    fn processing_to_string() {
        assert_eq!(ProcessingStatus::Processing.to_string(), "processing");
    }

    #[wasm_bindgen_test]
    fn processed_to_string() {
        assert_eq!(ProcessingStatus::Processed.to_string(), "processed");
    }

    #[wasm_bindgen_test]
    fn failed_to_string() {
        assert_eq!(ProcessingStatus::Failed.to_string(), "failed");
    }
}
