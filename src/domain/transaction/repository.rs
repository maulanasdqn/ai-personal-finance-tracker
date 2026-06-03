pub struct TransactionFilter {
    pub workspace_id: String,
    pub category: Option<String>,
    pub transaction_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
