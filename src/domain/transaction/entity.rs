use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub workspace_id: String,
    pub amount: f64,
    pub currency: String,
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
    pub source: TransactionSource,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct NewTransaction {
    pub id: String,
    pub workspace_id: String,
    pub amount: f64,
    pub currency: String,
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
    pub source: TransactionSource,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Income,
    Expense,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Income => write!(f, "income"),
            Self::Expense => write!(f, "expense"),
        }
    }
}

impl std::str::FromStr for TransactionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income" => Ok(Self::Income),
            "expense" => Ok(Self::Expense),
            _ => Err(format!("unknown type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionSource {
    Manual,
    Imported,
}

impl std::fmt::Display for TransactionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Imported => write!(f, "imported"),
        }
    }
}

impl std::str::FromStr for TransactionSource {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),
            "imported" => Ok(Self::Imported),
            _ => Err(format!("unknown source: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;
    use std::str::FromStr;

    #[wasm_bindgen_test]
    fn transaction_type_from_str_income() {
        assert_eq!(TransactionType::from_str("income").unwrap(), TransactionType::Income);
    }

    #[wasm_bindgen_test]
    fn transaction_type_from_str_expense() {
        assert_eq!(TransactionType::from_str("expense").unwrap(), TransactionType::Expense);
    }

    #[wasm_bindgen_test]
    fn transaction_type_from_str_unknown_is_err() {
        assert!(TransactionType::from_str("transfer").is_err());
    }

    #[wasm_bindgen_test]
    fn transaction_type_to_string_income() {
        assert_eq!(TransactionType::Income.to_string(), "income");
    }

    #[wasm_bindgen_test]
    fn transaction_type_to_string_expense() {
        assert_eq!(TransactionType::Expense.to_string(), "expense");
    }
}
