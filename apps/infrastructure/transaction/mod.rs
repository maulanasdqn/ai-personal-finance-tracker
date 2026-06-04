use crate::domain::transaction::entity::{
    NewTransaction, Transaction, TransactionCategory, TransactionSource, TransactionType,
};
use crate::domain::transaction::repository::{TransactionFilter, TransactionRepository};
use crate::error::AppError;
use rust_d1_orm::{opt_js, D1Model, Order, Query, Table};
use serde::Deserialize;
use std::str::FromStr;
use worker::{wasm_bindgen::JsValue, D1Database};

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

impl D1Model for TransactionRow {
    const TABLE: &'static str = "transactions";
    const COLUMNS: &'static [&'static str] = &[
        "id",
        "workspace_id",
        "amount",
        "currency",
        "category",
        "description",
        "transaction_date",
        "transaction_type",
        "source",
        "created_by",
        "created_at",
        "updated_at",
    ];
    fn values(&self) -> Vec<JsValue> {
        vec![
            self.id.clone().into(),
            self.workspace_id.clone().into(),
            self.amount.into(),
            self.currency.clone().into(),
            self.category.clone().into(),
            opt_js(self.description.clone()),
            self.transaction_date.clone().into(),
            self.transaction_type.clone().into(),
            self.source.clone().into(),
            self.created_by.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
        ]
    }
}

impl From<TransactionRow> for Transaction {
    fn from(r: TransactionRow) -> Self {
        Self {
            id: r.id,
            workspace_id: r.workspace_id,
            amount: r.amount,
            currency: r.currency,
            category: TransactionCategory::from_str(&r.category)
                .unwrap_or(TransactionCategory::Other),
            description: r.description,
            transaction_date: r.transaction_date,
            transaction_type: TransactionType::from_str(&r.transaction_type)
                .unwrap_or(TransactionType::Expense),
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
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }
}

impl TransactionRepository for D1TransactionRepository {
    async fn create(&self, tx: NewTransaction) -> Result<Transaction, AppError> {
        let row = TransactionRow {
            id: tx.id,
            workspace_id: tx.workspace_id,
            amount: tx.amount,
            currency: tx.currency,
            category: tx.category.to_string(),
            description: tx.description,
            transaction_date: tx.transaction_date,
            transaction_type: tx.transaction_type.to_string(),
            source: tx.source.to_string(),
            created_by: tx.created_by,
            created_at: tx.created_at,
            updated_at: tx.updated_at,
        };
        Table::<TransactionRow>::new(&self.db)
            .insert(&row)
            .await
            .map_err(|_| AppError::Internal)
            .map(Into::into)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Transaction>, AppError> {
        Table::<TransactionRow>::new(&self.db)
            .find_one(Query::new().eq("id", id))
            .await
            .map_err(|_| AppError::Internal)
            .map(|r| r.map(Into::into))
    }

    async fn list(&self, f: TransactionFilter) -> Result<Vec<Transaction>, AppError> {
        let query = Query::new()
            .eq("workspace_id", f.workspace_id)
            .filter_optional("category", f.category)
            .filter_optional("transaction_type", f.transaction_type)
            .filter_optional_gte("transaction_date", f.date_from)
            .filter_optional_lte("transaction_date", f.date_to)
            .order_by("transaction_date", Order::Desc)
            .limit(f.limit.unwrap_or(50).into())
            .offset(f.offset.unwrap_or(0).into());
        Table::<TransactionRow>::new(&self.db)
            .find_all(query)
            .await
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    async fn delete(&self, id: &str) -> Result<(), AppError> {
        Table::<TransactionRow>::new(&self.db)
            .delete(Query::new().eq("id", id))
            .await
            .map_err(|_| AppError::Internal)
    }
}
