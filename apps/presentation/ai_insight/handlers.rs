use crate::application::ai_insight::use_cases::{dto::GenerateInsightsInput, generate};
use crate::domain::ai_insight::repository::AiInsightRepository;
use crate::domain::workspace::repository::WorkspaceRepository;
use crate::error::AppError;
use crate::infrastructure::{ai_insight::D1AiInsightRepository, transaction::D1TransactionRepository, workspace::D1WorkspaceRepository};
use crate::presentation::{ai_insight::dto::*, guard, middleware::authenticate};
use worker::{Request, Response, RouteContext};

const AI_COOLDOWN_SECS: i64 = 300;

pub async fn list_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_list(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_list(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let wid = ctx.param("workspace_id").ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?;
    let secret = ctx.env.secret("JWT_SECRET").map_err(AppError::from)?.to_string();
    let user = authenticate(&req, &secret)?;
    D1WorkspaceRepository::new(ctx.env.d1("DB").map_err(AppError::from)?).find_member(wid, &user.user_id).await?.ok_or_else(|| AppError::Forbidden("not a member".into()))?;
    let url = req.url().map_err(AppError::from)?;
    let params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    let insight_type = params.get("type").map(String::as_str);
    let insights = D1AiInsightRepository::new(ctx.env.d1("DB").map_err(AppError::from)?).list_by_workspace(wid, insight_type).await?;
    let resp: Vec<AiInsightResponse> = insights.into_iter().map(Into::into).collect();
    Response::from_json(&resp).map_err(AppError::from)
}

pub async fn generate_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_generate(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_generate(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    guard::require_json(&req)?;
    guard::limit_body(&req)?;
    let wid = ctx.param("workspace_id").ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?.to_string();
    let secret = ctx.env.secret("JWT_SECRET").map_err(AppError::from)?.to_string();
    let user = authenticate(&req, &secret)?;
    D1WorkspaceRepository::new(ctx.env.d1("DB").map_err(AppError::from)?).find_member(&wid, &user.user_id).await?.ok_or_else(|| AppError::Forbidden("not a member".into()))?;
    let insight_repo = D1AiInsightRepository::new(ctx.env.d1("DB").map_err(AppError::from)?);
    if insight_repo.was_recently_generated(&wid, AI_COOLDOWN_SECS).await? {
        return Err(AppError::TooManyRequests("insights were already generated recently — please wait 5 minutes".into()));
    }
    let raw: serde_json::Value = req.json().await.map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    let body = GenerateInsightsRequest::validate_and_parse(&raw)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let api_key = ctx.env.secret("DEEPSEEK_API_KEY").map_err(AppError::from)?.to_string();
    let tx_repo = D1TransactionRepository::new(ctx.env.d1("DB").map_err(AppError::from)?);
    let insight_repo2 = D1AiInsightRepository::new(ctx.env.d1("DB").map_err(AppError::from)?);
    let insights = generate::execute(GenerateInsightsInput { workspace_id: wid, date_from: body.date_from, date_to: body.date_to }, &tx_repo, &insight_repo2, &api_key).await?;
    let resp: Vec<AiInsightResponse> = insights.into_iter().map(Into::into).collect();
    Response::from_json(&resp).map_err(AppError::from)
}
