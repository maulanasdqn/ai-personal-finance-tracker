use worker::{event, Context, Env, Request, Response, Router};

mod application;
mod domain;
mod error;
mod infrastructure;
mod presentation;

use presentation::{
    ai_insight::handlers as insight_handlers,
    auth::handlers as auth_handlers,
    bank_statement::handlers as stmt_handlers,
    docs::handlers as docs_handlers,
    transaction::handlers as tx_handlers,
    workspace::handlers as ws_handlers,
};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .post_async("/api/v1/auth/register", |r, c| async move { auth_handlers::register_handler(r, c).await })
        .post_async("/api/v1/auth/login", |r, c| async move { auth_handlers::login_handler(r, c).await })
        .get_async("/api/v1/workspaces", |r, c| async move { ws_handlers::list_handler(r, c).await })
        .post_async("/api/v1/workspaces", |r, c| async move { ws_handlers::create_handler(r, c).await })
        .get_async("/api/v1/workspaces/:id", |r, c| async move { ws_handlers::get_handler(r, c).await })
        .put_async("/api/v1/workspaces/:id", |r, c| async move { ws_handlers::update_handler(r, c).await })
        .delete_async("/api/v1/workspaces/:id", |r, c| async move { ws_handlers::delete_handler(r, c).await })
        .post_async("/api/v1/workspaces/:id/members", |r, c| async move { ws_handlers::invite_handler(r, c).await })
        .get_async("/api/v1/workspaces/:id/members", |r, c| async move { ws_handlers::members_handler(r, c).await })
        .get_async("/api/v1/workspaces/:workspace_id/transactions", |r, c| async move { tx_handlers::list_handler(r, c).await })
        .post_async("/api/v1/workspaces/:workspace_id/transactions", |r, c| async move { tx_handlers::create_handler(r, c).await })
        .get_async("/api/v1/workspaces/:workspace_id/transactions/:id", |r, c| async move { tx_handlers::get_handler(r, c).await })
        .delete_async("/api/v1/workspaces/:workspace_id/transactions/:id", |r, c| async move { tx_handlers::delete_handler(r, c).await })
        .get_async("/api/v1/workspaces/:workspace_id/statements", |r, c| async move { stmt_handlers::list_handler(r, c).await })
        .post_async("/api/v1/workspaces/:workspace_id/statements", |r, c| async move { stmt_handlers::upload_handler(r, c).await })
        .get_async("/api/v1/workspaces/:workspace_id/statements/:id", |r, c| async move { stmt_handlers::get_handler(r, c).await })
        .get_async("/api/v1/workspaces/:workspace_id/insights", |r, c| async move { insight_handlers::list_handler(r, c).await })
        .post_async("/api/v1/workspaces/:workspace_id/insights/generate", |r, c| async move { insight_handlers::generate_handler(r, c).await })
        .get_async("/api/docs/openapi.json", |r, c| async move { docs_handlers::spec_handler(r, c).await })
        .get_async("/api/docs", |r, c| async move { docs_handlers::ui_handler(r, c).await })
        .get_async("/", |_, _| async move { Response::ok("AI Finance Tracker API — docs at /api/docs") })
        .run(req, env)
        .await
}
