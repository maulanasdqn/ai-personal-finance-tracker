use crate::application::auth::{login, register};
use crate::error::AppError;
use crate::presentation::auth::dto::{AuthResponse, LoginRequest, RegisterRequest};
use worker::{Request, Response, RouteContext};

pub async fn register_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_register(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_register(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let body: RegisterRequest = req.json().await.map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    if body.email.is_empty() || body.password.len() < 8 {
        return Err(AppError::BadRequest("email required and password must be at least 8 chars".into()));
    }
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let secret = ctx.env.secret("JWT_SECRET").map_err(AppError::from)?.to_string();
    let repo = crate::infrastructure::repository::user::D1UserRepository::new(db);
    let out = register::execute(register::RegisterInput { email: body.email, password: body.password, full_name: body.full_name }, &repo, &secret).await?;
    Response::from_json(&AuthResponse { token: out.token, user_id: out.user_id, email: out.email, full_name: out.full_name })
        .map_err(AppError::from)
}

pub async fn login_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    handle_login(req, ctx).await.map(Ok).unwrap_or_else(|e| Ok(e.into_response()))
}

async fn handle_login(mut req: Request, ctx: RouteContext<()>) -> Result<Response, AppError> {
    let body: LoginRequest = req.json().await.map_err(|_| AppError::BadRequest("invalid JSON".into()))?;
    let db = ctx.env.d1("DB").map_err(AppError::from)?;
    let secret = ctx.env.secret("JWT_SECRET").map_err(AppError::from)?.to_string();
    let repo = crate::infrastructure::repository::user::D1UserRepository::new(db);
    let out = login::execute(login::LoginInput { email: body.email, password: body.password }, &repo, &secret).await?;
    Response::from_json(&AuthResponse { token: out.token, user_id: out.user_id, email: out.email, full_name: out.full_name })
        .map_err(AppError::from)
}
