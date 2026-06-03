pub struct CreateTransactionInput {
    pub workspace_id: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: String,
    pub transaction_type: String,
    pub created_by: String,
}
