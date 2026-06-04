use crate::application::auth::{
    dto::{LoginInput, RegisterInput},
    login, register,
};
use crate::application::validation::validate_password_strength;
use crate::error::AppError;
use crate::presentation::auth::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::presentation::guard;
use worker::{Request, Response, RouteContext};

pub async fn register_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_register(req, ctx)
        .await
        .map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_register(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    guard::require_json(&req)?;
    guard::limit_body(&req)?;
    let raw: serde_json::Value = req
        .json()
        .await
        .map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    let body = RegisterRequest::validate_and_parse(&raw)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    validate_password_strength(&body.password)?;
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let secret = ctx
        .env
        .secret("JWT_SECRET")
        .map_err(AppError::from)?
        .to_string();
    let repo = crate::infrastructure::user::D1UserRepository::new(db);
    let out = register::execute(
        RegisterInput {
            email: body.email,
            password: body.password,
            full_name: body.full_name,
        },
        &repo,
        &secret,
    )
    .await?;
    Response::from_json(&AuthResponse {
        token: out.token,
        user_id: out.user_id,
        email: out.email,
        full_name: out.full_name,
    })
    .map_err(AppError::from)
}

pub async fn login_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_login(req, ctx)
        .await
        .map_or_else(|e| Ok(e.into_response()), Ok)
}

async fn handle_login(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    guard::require_json(&req)?;
    guard::limit_body(&req)?;
    let raw: serde_json::Value = req
        .json()
        .await
        .map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    let body =
        LoginRequest::validate_and_parse(&raw).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let secret = ctx
        .env
        .secret("JWT_SECRET")
        .map_err(AppError::from)?
        .to_string();
    let repo = crate::infrastructure::user::D1UserRepository::new(db);
    let out = login::execute(
        LoginInput {
            email: body.email,
            password: body.password,
        },
        &repo,
        &secret,
    )
    .await?;
    Response::from_json(&AuthResponse {
        token: out.token,
        user_id: out.user_id,
        email: out.email,
        full_name: out.full_name,
    })
    .map_err(AppError::from)
}
