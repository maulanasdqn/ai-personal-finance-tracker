pub mod deepseek;

pub const FINANCIAL_TIPS_PROMPT: &str = r#"You are a financial advisor AI. Based on the transaction data below, provide:
1. 3-5 actionable financial tips to improve savings
2. Top spending categories to reduce with specific percentage targets
3. 2-3 product/service recommendations that could save money

Respond in JSON format:
{
  "tips": [{"title": "...", "content": "..."}],
  "reductions": [{"category": "...", "current_percentage": 0, "target_percentage": 0, "advice": "..."}],
  "recommendations": [{"title": "...", "reason": "...", "estimated_savings": "..."}]
}

Transaction summary:
"#;

pub const BANK_STATEMENT_PROMPT: &str = r#"Extract all transactions from this bank statement. For each transaction provide:
- date (YYYY-MM-DD format)
- amount (positive number)
- type (income or expense)
- description
- category (one of: Food, Transport, Shopping, Entertainment, Health, Education, Utilities, Salary, Investment, Other)

Respond ONLY with a valid JSON array:
[{"date": "...", "amount": 0, "type": "income|expense", "description": "...", "category": "..."}]
"#;
