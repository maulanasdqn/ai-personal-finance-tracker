use crate::domain::transaction::entity::{NewTransaction, Transaction, TransactionSource, TransactionType};
use crate::domain::transaction::repository::TransactionFilter;
use crate::error::AppError;
use serde::Deserialize;
use std::str::FromStr;
use worker::D1Database;

#[derive(Deserialize)]
struct TransactionRow {
    id: String,
    workspace_id: String,
    amount: f64,
    currency: String,
    category: String,
    description: Option<String>,
    transaction_date: String,
    transaction_type: String,
    source: String,
    created_by: String,
    created_at: String,
    updated_at: String,
}

impl From<TransactionRow> for Transaction {
    fn from(r: TransactionRow) -> Self {
        Self {
            id: r.id,
            workspace_id: r.workspace_id,
            amount: r.amount,
            currency: r.currency,
            category: r.category,
            description: r.description,
            transaction_date: r.transaction_date,
            transaction_type: TransactionType::from_str(&r.transaction_type).unwrap_or(TransactionType::Expense),
            source: TransactionSource::from_str(&r.source).unwrap_or(TransactionSource::Manual),
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

pub struct D1TransactionRepository {
    db: D1Database,
}

impl D1TransactionRepository {
    pub fn new(db: D1Database) -> Self { Self { db } }

    pub async fn create(&self, tx: NewTransaction) -> Result<Transaction, AppError> {
        self.db
            .prepare("INSERT INTO transactions (id, workspace_id, amount, currency, category, description, transaction_date, transaction_type, source, created_by, created_at, updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)")
            .bind(&[
                tx.id.clone().into(), tx.workspace_id.into(), tx.amount.into(), tx.currency.into(),
                tx.category.into(), tx.description.into(), tx.transaction_date.into(),
                tx.transaction_type.to_string().into(), tx.source.to_string().into(),
                tx.created_by.into(), tx.created_at.into(), tx.updated_at.into(),
            ])
            .map_err(|_| AppError::Internal)?
            .run().await.map_err(|_| AppError::Internal)?;
        self.find_by_id(&tx.id).await?.ok_or_else(|| AppError::Internal)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Transaction>, AppError> {
        self.db.prepare("SELECT * FROM transactions WHERE id = ?1")
            .bind(&[id.into()]).map_err(|_| AppError::Internal)?
            .first::<TransactionRow>(None).await
            .map_err(|_| AppError::Internal).map(|r| r.map(Into::into))
    }

    pub async fn list(&self, f: TransactionFilter) -> Result<Vec<Transaction>, AppError> {
        let limit = f.limit.unwrap_or(50);
        let offset = f.offset.unwrap_or(0);
        let results = self.db
            .prepare("SELECT * FROM transactions WHERE workspace_id = ?1 AND (?2 IS NULL OR category = ?2) AND (?3 IS NULL OR transaction_type = ?3) AND (?4 IS NULL OR transaction_date >= ?4) AND (?5 IS NULL OR transaction_date <= ?5) ORDER BY transaction_date DESC LIMIT ?6 OFFSET ?7")
            .bind(&[
                f.workspace_id.into(), f.category.into(), f.transaction_type.into(),
                f.date_from.into(), f.date_to.into(), limit.into(), offset.into(),
            ])
            .map_err(|_| AppError::Internal)?
            .all().await.map_err(|_| AppError::Internal)?;
        results.results::<TransactionRow>()
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        self.db.prepare("DELETE FROM transactions WHERE id = ?1")
            .bind(&[id.into()]).map_err(|_| AppError::Internal)?
            .run().await.map_err(|_| AppError::Internal)?;
        Ok(())
    }
}
