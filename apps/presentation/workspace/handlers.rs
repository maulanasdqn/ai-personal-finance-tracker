use crate::application::workspace::use_cases::{create, dto::{CreateWorkspaceInput, InviteMemberInput}, invite};
use crate::domain::workspace::entity::WorkspacePatch;
use crate::domain::workspace::repository::WorkspaceRepository;
use crate::error::AppError;
use crate::infrastructure::{user::D1UserRepository, workspace::D1WorkspaceRepository};
use crate::presentation::workspace::dto::{CreateWorkspaceRequest, InviteMemberRequest, MemberResponse, UpdateWorkspaceRequest, WorkspaceResponse};
use crate::presentation::{guard, middleware::authenticate};
use worker::{Request, Response, RouteContext};

macro_rules! auth {
    ($req:expr, $ctx:expr) => {{
        let secret = $ctx.env.secret("JWT_SECRET").map_err(AppError::from)?.to_string();
        authenticate(&$req, &secret)?
    }};
}

macro_rules! workspace_repo {
    ($ctx:expr) => {
        D1WorkspaceRepository::new($ctx.env.d1("DB").map_err(AppError::from)?)
    };
}

pub async fn list_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_list(req, ctx).await.map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_list(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let user = auth!(req, ctx);
    let repo = workspace_repo!(ctx);
    let workspaces = repo.list_by_user(&user.user_id).await?;
    let resp: Vec<WorkspaceResponse> = workspaces.into_iter().map(Into::into).collect();
    Response::from_json(&resp).map_err(AppError::from)
}

pub async fn create_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_create(req, ctx).await.map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_create(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    guard::require_json(&req)?;
    guard::limit_body(&req)?;
    let user = auth!(req, ctx);
    let raw: serde_json::Value = req.json().await.map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    let body = CreateWorkspaceRequest::validate_and_parse(&raw)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let repo = workspace_repo!(ctx);
    let workspace = create::execute(CreateWorkspaceInput { name: body.name, description: body.description, owner_id: user.user_id }, &repo).await?;
    Response::from_json(&WorkspaceResponse::from(workspace)).map_err(AppError::from)
}

pub async fn get_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_get(req, ctx).await.map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_get(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let user = auth!(req, ctx);
    let id = ctx.param("id").ok_or_else(|| AppError::BadRequest("missing id".into()))?;
    let repo = workspace_repo!(ctx);
    repo.find_member(id, &user.user_id).await?.ok_or_else(|| AppError::Forbidden("not a member".into()))?;
    let workspace = repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("workspace not found".into()))?;
    Response::from_json(&WorkspaceResponse::from(workspace)).map_err(AppError::from)
}

pub async fn update_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_update(req, ctx).await.map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_update(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    guard::require_json(&req)?;
    guard::limit_body(&req)?;
    let user = auth!(req, ctx);
    let id = ctx.param("id").ok_or_else(|| AppError::BadRequest("missing id".into()))?;
    let body: UpdateWorkspaceRequest = req.json().await.map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    if let Some(ref name) = body.name {
        if name.is_empty() || name.len() > 80 {
            return Err(AppError::BadRequest("workspace name must be between 1 and 80 characters".into()));
        }
    }
    let repo = workspace_repo!(ctx);
    let member = repo.find_member(id, &user.user_id).await?.ok_or_else(|| AppError::Forbidden("not a member".into()))?;
    if member.role == crate::domain::workspace::entity::MemberRole::Member {
        return Err(AppError::Forbidden("insufficient permissions".into()));
    }
    let now = chrono::Utc::now().to_rfc3339();
    let workspace = repo.update(id, WorkspacePatch { name: body.name, description: body.description }, &now).await?;
    Response::from_json(&WorkspaceResponse::from(workspace)).map_err(AppError::from)
}

pub async fn delete_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_delete(req, ctx).await.map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_delete(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let user = auth!(req, ctx);
    let id = ctx.param("id").ok_or_else(|| AppError::BadRequest("missing id".into()))?;
    let repo = workspace_repo!(ctx);
    let workspace = repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("workspace not found".into()))?;
    if workspace.owner_id != user.user_id { return Err(AppError::Forbidden("only the owner can delete".into())); }
    repo.delete(id).await?;
    Response::ok("deleted").map_err(AppError::from)
}

pub async fn invite_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_invite(req, ctx).await.map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_invite(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    guard::require_json(&req)?;
    guard::limit_body(&req)?;
    let user = auth!(req, ctx);
    let workspace_id = ctx.param("id").ok_or_else(|| AppError::BadRequest("missing id".into()))?;
    let raw: serde_json::Value = req.json().await.map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    let body = InviteMemberRequest::validate_and_parse(&raw)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let workspace_repo = D1WorkspaceRepository::new(ctx.env.d1("DB").map_err(AppError::from)?);
    let user_repo = D1UserRepository::new(ctx.env.d1("DB").map_err(AppError::from)?);
    let member = invite::execute(InviteMemberInput { workspace_id: workspace_id.to_string(), inviter_id: user.user_id, email: body.email }, &workspace_repo, &user_repo).await?;
    Response::from_json(&MemberResponse::from(member)).map_err(AppError::from)
}

pub async fn members_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_members(req, ctx).await.map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_members(req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let user = auth!(req, ctx);
    let id = ctx.param("id").ok_or_else(|| AppError::BadRequest("missing id".into()))?;
    let repo = workspace_repo!(ctx);
    repo.find_member(id, &user.user_id).await?.ok_or_else(|| AppError::Forbidden("not a member".into()))?;
    let members = repo.list_members(id).await?;
    let resp: Vec<MemberResponse> = members.into_iter().map(Into::into).collect();
    Response::from_json(&resp).map_err(AppError::from)
}
