use super::dto::CreateTransactionInput;
use crate::domain::transaction::entity::{NewTransaction, Transaction, TransactionSource, TransactionType};
use crate::error::AppError;
use crate::infrastructure::repository::transaction::D1TransactionRepository;
use std::str::FromStr;
use uuid::Uuid;

pub async fn execute(input: CreateTransactionInput, repo: &D1TransactionRepository) -> Result<Transaction, AppError> {
    let now = chrono::Utc::now().to_rfc3339();
    let tx_type = TransactionType::from_str(&input.transaction_type)
        .map_err(|_| AppError::BadRequest("transaction_type must be 'income' or 'expense'".into()))?;
    repo.create(NewTransaction {
        id: Uuid::new_v4().to_string(),
        workspace_id: input.workspace_id,
        amount: input.amount,
        currency: input.currency.unwrap_or_else(|| "IDR".into()),
        category: input.category,
        description: input.description,
        transaction_date: input.transaction_date,
        transaction_type: tx_type,
        source: TransactionSource::Manual,
        created_by: input.created_by,
        created_at: now.clone(),
        updated_at: now,
    }).await
}
