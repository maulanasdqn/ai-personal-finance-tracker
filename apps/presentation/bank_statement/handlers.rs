use crate::application::bank_statement::use_cases::{dto::UploadInput, upload};
use crate::application::validation::{sanitize_file_name, validate_file_upload};
use crate::domain::bank_statement::repository::BankStatementRepository;
use crate::domain::workspace::repository::WorkspaceRepository;
use crate::error::AppError;
use crate::infrastructure::{
    bank_statement::D1BankStatementRepository, workspace::D1WorkspaceRepository,
};
use crate::presentation::{bank_statement::dto::BankStatementResponse, middleware::authenticate};
use worker::{Request, Response, RouteContext};

pub async fn list_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_list(req, ctx)
        .await
        .map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_list(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let wid = ctx
        .param("workspace_id")
        .ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?;
    let secret = ctx
        .env
        .secret("JWT_SECRET")
        .map_err(AppError::from)?
        .to_string();
    let user = authenticate(&req, &secret)?;
    D1WorkspaceRepository::new(ctx.env.d1("DB").map_err(AppError::from)?)
        .find_member(wid, &user.user_id)
        .await?
        .ok_or_else(|| AppError::Forbidden("not a member".into()))?;
    let stmts = D1BankStatementRepository::new(ctx.env.d1("DB").map_err(AppError::from)?)
        .list_by_workspace(wid)
        .await?;
    let resp: Vec<BankStatementResponse> = stmts.into_iter().map(Into::into).collect();
    Response::from_json(&resp).map_err(AppError::from)
}

pub async fn upload_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_upload(req, ctx)
        .await
        .map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_upload(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let wid = ctx
        .param("workspace_id")
        .ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?
        .to_string();
    let secret = ctx
        .env
        .secret("JWT_SECRET")
        .map_err(AppError::from)?
        .to_string();
    let user = authenticate(&req, &secret)?;
    D1WorkspaceRepository::new(ctx.env.d1("DB").map_err(AppError::from)?)
        .find_member(&wid, &user.user_id)
        .await?
        .ok_or_else(|| AppError::Forbidden("not a member".into()))?;

    let content_type = req
        .headers()
        .get("content-type")
        .map_err(AppError::from)?
        .unwrap_or_else(|| "application/octet-stream".into());

    let raw_file_name = req
        .headers()
        .get("x-file-name")
        .map_err(AppError::from)?
        .unwrap_or_else(|| "statement".into());
    let file_name = sanitize_file_name(&raw_file_name);

    let bytes = req.bytes().await.map_err(AppError::from)?;
    validate_file_upload(&bytes, &content_type)?;

    let bucket = ctx.env.bucket("STORAGE").map_err(AppError::from)?;
    let api_key = ctx
        .env
        .secret("DEEPSEEK_API_KEY")
        .map_err(AppError::from)?
        .to_string();
    let repo = D1BankStatementRepository::new(ctx.env.d1("DB").map_err(AppError::from)?);

    let stmt = upload::execute(
        UploadInput {
            workspace_id: wid,
            file_name,
            file_data: bytes,
            content_type,
            created_by: user.user_id,
        },
        &repo,
        &bucket,
        &api_key,
    )
    .await?;

    Response::from_json(&BankStatementResponse::from(stmt)).map_err(AppError::from)
}

pub async fn get_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_get(req, ctx)
        .await
        .map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_get(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let wid = ctx
        .param("workspace_id")
        .ok_or_else(|| AppError::BadRequest("missing workspace_id".into()))?;
    let sid = ctx
        .param("id")
        .ok_or_else(|| AppError::BadRequest("missing id".into()))?;
    let secret = ctx
        .env
        .secret("JWT_SECRET")
        .map_err(AppError::from)?
        .to_string();
    let user = authenticate(&req, &secret)?;
    D1WorkspaceRepository::new(ctx.env.d1("DB").map_err(AppError::from)?)
        .find_member(wid, &user.user_id)
        .await?
        .ok_or_else(|| AppError::Forbidden("not a member".into()))?;
    let stmt = D1BankStatementRepository::new(ctx.env.d1("DB").map_err(AppError::from)?)
        .find_by_id(sid)
        .await?
        .ok_or_else(|| AppError::NotFound("statement not found".into()))?;
    Response::from_json(&BankStatementResponse::from(stmt)).map_err(AppError::from)
}
