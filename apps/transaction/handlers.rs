use crate::application::transaction::use_cases::{create as create_uc, dto::CreateTransactionInput};
use crate::domain::transaction::repository::{TransactionFilter, TransactionRepository};
use crate::domain::workspace::repository::WorkspaceRepository;
use crate::error::AppError;
use crate::infrastructure::repository::{transaction::D1TransactionRepository, workspace::D1WorkspaceRepository};
use crate::{guard, middleware::authenticate, transaction::dto::*};
use worker::{Request, Response, RouteContext};

macro_rules! auth_member {
    ($req:expr, $ctx:expr, $workspace_id:expr) => {{
        let secret = $ctx.env.secret("JWT_SECRET").map_err(AppError::from)?.to_string();
        let user = authenticate(&$req, &secret)?;
        let db = $ctx.env.d1("DB").map_err(AppError::from)?;
        let ws_repo = D1WorkspaceRepository::new(db);
        ws_repo.find_member($workspace_id, &user.user_id).await?.ok_or_else(|| AppError::Forbidden("not a workspace member".into()))?;
        user
    }};
}

pub async fn list_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_list(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_list(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let wid = ctx.param("workspace_id").ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?;
    let _user = auth_member!(req, ctx, wid);
    let url = req.url().map_err(AppError::from)?;
    let params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let repo = D1TransactionRepository::new(db);
    let txs = repo.list(TransactionFilter {
        workspace_id: wid.to_string(),
        category: params.get("category").cloned(),
        transaction_type: params.get("type").cloned(),
        date_from: params.get("from").cloned(),
        date_to: params.get("to").cloned(),
        limit: params.get("limit").and_then(|v| v.parse().ok()),
        offset: params.get("offset").and_then(|v| v.parse().ok()),
    }).await?;
    let resp: Vec<TransactionResponse> = txs.into_iter().map(Into::into).collect();
    Response::from_json(&resp).map_err(AppError::from)
}

pub async fn create_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_create(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_create(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    guard::require_json(&req)?;
    guard::limit_body(&req)?;
    let wid = ctx.param("workspace_id").ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?.to_string();
    let user = auth_member!(req, ctx, &wid);
    let body: CreateTransactionRequest = req.json().await.map_err(|_| AppError::BadRequest("invalid request body".into()))?;
    if body.amount < 0.01 || body.amount > 1_000_000_000_000.0 {
        return Err(AppError::BadRequest("amount must be between 0.01 and 1,000,000,000,000".into()));
    }
    if !is_valid_date(&body.transaction_date) {
        return Err(AppError::BadRequest("transaction_date must be in YYYY-MM-DD format".into()));
    }
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let repo = D1TransactionRepository::new(db);
    let tx = create_uc::execute(CreateTransactionInput {
        workspace_id: wid, amount: body.amount, currency: body.currency,
        category: body.category, description: body.description,
        transaction_date: body.transaction_date, transaction_type: body.transaction_type,
        created_by: user.user_id,
    }, &repo).await?;
    Response::from_json(&TransactionResponse::from(tx)).map_err(AppError::from)
}

pub async fn get_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_get(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_get(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let wid = ctx.param("workspace_id").ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?;
    let txid = ctx.param("id").ok_or_else(|| AppError::BadRequest("missing id".into()))?;
    auth_member!(req, ctx, wid);
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let repo = D1TransactionRepository::new(db);
    let tx = repo.find_by_id(txid).await?.ok_or_else(|| AppError::NotFound("transaction not found".into()))?;
    Response::from_json(&TransactionResponse::from(tx)).map_err(AppError::from)
}

pub async fn delete_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_delete(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_delete(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let wid = ctx.param("workspace_id").ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?;
    let txid = ctx.param("id").ok_or_else(|| AppError::BadRequest("missing id".into()))?.to_string();
    auth_member!(req, ctx, wid);
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let repo = D1TransactionRepository::new(db);
    repo.delete(&txid).await?;
    Response::ok("deleted").map_err(AppError::from)
}

fn is_valid_date(s: &str) -> bool {
    s.len() == 10
        && s.as_bytes()[4] == b'-'
        && s.as_bytes()[7] == b'-'
        && s[..4].parse::<u32>().is_ok()
        && s[5..7].parse::<u32>().is_ok()
        && s[8..10].parse::<u32>().is_ok()
}
