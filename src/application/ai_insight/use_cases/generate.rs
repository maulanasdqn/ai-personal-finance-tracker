use super::dto::GenerateInsightsInput;
use crate::domain::ai_insight::entity::{AiInsight, InsightType, NewAiInsight};
use crate::domain::transaction::repository::TransactionFilter;
use crate::error::AppError;
use crate::infrastructure::{ai, repository::{ai_insight::D1AiInsightRepository, transaction::D1TransactionRepository}};
use uuid::Uuid;

pub async fn execute(input: GenerateInsightsInput, tx_repo: &D1TransactionRepository, insight_repo: &D1AiInsightRepository, api_key: &str) -> Result<Vec<AiInsight>, AppError> {
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
    let prompt = format!("{}{}", ai::FINANCIAL_TIPS_PROMPT, summary);
    let ai_response = ai::deepseek::analyze_text(&prompt, api_key).await?;

    let parsed: serde_json::Value = serde_json::from_str(&ai_response)
        .map_err(|_| AppError::Internal)?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut results = vec![];

    if let Some(tips) = parsed["tips"].as_array() {
        for tip in tips {
            let insight = insight_repo.create(NewAiInsight {
                id: Uuid::new_v4().to_string(),
                workspace_id: input.workspace_id.clone(),
                insight_type: InsightType::Tip,
                title: tip["title"].as_str().unwrap_or("Financial Tip").to_string(),
                content: tip["content"].as_str().unwrap_or("").to_string(),
                metadata: None,
                created_at: now.clone(),
            }).await?;
            results.push(insight);
        }
    }

    if let Some(reductions) = parsed["reductions"].as_array() {
        for item in reductions {
            let insight = insight_repo.create(NewAiInsight {
                id: Uuid::new_v4().to_string(),
                workspace_id: input.workspace_id.clone(),
                insight_type: InsightType::Reduction,
                title: format!("Reduce {} spending", item["category"].as_str().unwrap_or("")),
                content: item["advice"].as_str().unwrap_or("").to_string(),
                metadata: Some(item.clone()),
                created_at: now.clone(),
            }).await?;
            results.push(insight);
        }
    }

    if let Some(recs) = parsed["recommendations"].as_array() {
        for rec in recs {
            let insight = insight_repo.create(NewAiInsight {
                id: Uuid::new_v4().to_string(),
                workspace_id: input.workspace_id.clone(),
                insight_type: InsightType::Recommendation,
                title: rec["title"].as_str().unwrap_or("Recommendation").to_string(),
                content: rec["reason"].as_str().unwrap_or("").to_string(),
                metadata: Some(rec.clone()),
                created_at: now.clone(),
            }).await?;
            results.push(insight);
        }
    }

    Ok(results)
}

fn summarize_transactions(transactions: &[crate::domain::transaction::entity::Transaction]) -> String {
    let total_income: f64 = transactions.iter().filter(|t| t.transaction_type == crate::domain::transaction::entity::TransactionType::Income).map(|t| t.amount).sum();
    let total_expense: f64 = transactions.iter().filter(|t| t.transaction_type == crate::domain::transaction::entity::TransactionType::Expense).map(|t| t.amount).sum();
    let mut by_category: std::collections::HashMap<&str, f64> = std::collections::HashMap::new();
    for tx in transactions {
        *by_category.entry(tx.category.as_str()).or_insert(0.0) += tx.amount;
    }
    let mut cats: Vec<_> = by_category.iter().collect();
    cats.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let top_cats: String = cats.iter().take(5).map(|(k, v)| format!("{}: {:.0}", k, v)).collect::<Vec<_>>().join(", ");
    format!("Total income: {:.0}, Total expenses: {:.0}, Net: {:.0}. Top categories: {}. Transaction count: {}", total_income, total_expense, total_income - total_expense, top_cats, transactions.len())
}
