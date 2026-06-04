use super::dto::GenerateInsightsInput;
use crate::domain::ai_insight::entity::{AiInsight, InsightType, NewAiInsight};
use crate::domain::ai_insight::repository::AiInsightRepository;
use crate::domain::transaction::repository::{TransactionFilter, TransactionRepository};
use crate::error::AppError;
use crate::infrastructure::ai;
use uuid::Uuid;

pub async fn execute(input: GenerateInsightsInput, tx_repo: &impl TransactionRepository, insight_repo: &impl AiInsightRepository, api_key: &str) -> Result<Vec<AiInsight>, AppError> {
    let transactions = tx_repo.list(TransactionFilter {
        workspace_id: input.workspace_id.clone(),
        category: None,
        transaction_type: None,
        date_from: Some(input.date_from.clone()),
        date_to: Some(input.date_to.clone()),
        limit: Some(200),
        offset: None,
    }).await?;

    if transactions.is_empty() {
        return Err(AppError::BadRequest("no transactions found in the specified period".into()));
    }

    let summary = summarize_transactions(&transactions);
    let prompt = format!("{}{summary}", ai::FINANCIAL_TIPS_PROMPT);
    let ai_response = ai::deepseek::analyze_text(&prompt, api_key).await?;

    let parsed: serde_json::Value = serde_json::from_str(&ai_response)
        .map_err(|_| AppError::Internal)?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut batch: Vec<NewAiInsight> = vec![];

    if let Some(tips) = parsed["tips"].as_array() {
        for tip in tips {
            batch.push(NewAiInsight {
                id: Uuid::new_v4().to_string(),
                workspace_id: input.workspace_id.clone(),
                insight_type: InsightType::Tip,
                title: tip["title"].as_str().unwrap_or("Financial Tip").to_string(),
                content: tip["content"].as_str().unwrap_or("").to_string(),
                metadata: None,
                created_at: now.clone(),
            });
        }
    }

    if let Some(reductions) = parsed["reductions"].as_array() {
        for item in reductions {
            batch.push(NewAiInsight {
                id: Uuid::new_v4().to_string(),
                workspace_id: input.workspace_id.clone(),
                insight_type: InsightType::Reduction,
                title: { let cat = item["category"].as_str().unwrap_or(""); format!("Reduce {cat} spending") },
                content: item["advice"].as_str().unwrap_or("").to_string(),
                metadata: Some(item.clone()),
                created_at: now.clone(),
            });
        }
    }

    if let Some(recs) = parsed["recommendations"].as_array() {
        for rec in recs {
            batch.push(NewAiInsight {
                id: Uuid::new_v4().to_string(),
                workspace_id: input.workspace_id.clone(),
                insight_type: InsightType::Recommendation,
                title: rec["title"].as_str().unwrap_or("Recommendation").to_string(),
                content: rec["reason"].as_str().unwrap_or("").to_string(),
                metadata: Some(rec.clone()),
                created_at: now.clone(),
            });
        }
    }

    insight_repo.create_batch(batch).await
}

fn summarize_transactions(transactions: &[crate::domain::transaction::entity::Transaction]) -> String {
    let total_income: f64 = transactions.iter().filter(|t| t.transaction_type == crate::domain::transaction::entity::TransactionType::Income).map(|t| t.amount).sum();
    let total_expense: f64 = transactions.iter().filter(|t| t.transaction_type == crate::domain::transaction::entity::TransactionType::Expense).map(|t| t.amount).sum();
    let mut by_category: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for tx in transactions {
        *by_category.entry(tx.category.to_string()).or_insert(0.0) += tx.amount;
    }
    let mut cats: Vec<_> = by_category.iter().collect();
    cats.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let top_cats: String = cats.iter().take(5).map(|(k, v)| format!("{k}: {v:.0}")).collect::<Vec<_>>().join(", ");
    let net = total_income - total_expense;
    let count = transactions.len();
    format!("Total income: {total_income:.0}, Total expenses: {total_expense:.0}, Net: {net:.0}. Top categories: {top_cats}. Transaction count: {count}")
}
