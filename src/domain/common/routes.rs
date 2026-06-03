pub struct AuthRoutes;
pub struct WorkspaceRoutes;
pub struct TransactionRoutes;
pub struct BankStatementRoutes;
pub struct AiInsightRoutes;
pub struct DocsRoutes;

impl AuthRoutes {
    pub const REGISTER: &'static str = "/api/v1/auth/register";
    pub const LOGIN: &'static str = "/api/v1/auth/login";
}

impl WorkspaceRoutes {
    pub const BASE: &'static str = "/api/v1/workspaces";
    pub const BY_ID: &'static str = "/api/v1/workspaces/:id";
    pub const MEMBERS: &'static str = "/api/v1/workspaces/:id/members";
}

impl TransactionRoutes {
    pub const BASE: &'static str = "/api/v1/workspaces/:workspace_id/transactions";
    pub const BY_ID: &'static str = "/api/v1/workspaces/:workspace_id/transactions/:id";
}

impl BankStatementRoutes {
    pub const BASE: &'static str = "/api/v1/workspaces/:workspace_id/statements";
    pub const BY_ID: &'static str = "/api/v1/workspaces/:workspace_id/statements/:id";
}

impl AiInsightRoutes {
    pub const BASE: &'static str = "/api/v1/workspaces/:workspace_id/insights";
    pub const GENERATE: &'static str = "/api/v1/workspaces/:workspace_id/insights/generate";
}

impl DocsRoutes {
    pub const SPEC: &'static str = "/api/docs/openapi.json";
    pub const UI: &'static str = "/api/docs";
}
