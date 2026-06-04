use worker::{event, Context, Env, Method, Request, Response, Router};

mod ai_insight;
mod application;
mod auth;
mod bank_statement;
mod docs;
mod domain;
mod error;
mod guard;
mod infrastructure;
mod middleware;
mod security;
mod transaction;
mod workspace;

use ai_insight::routes as insight_routes;
use auth::routes as auth_routes;
use bank_statement::routes as stmt_routes;
use docs::routes as docs_routes;
use security::apply_security_headers;
use transaction::routes as tx_routes;
use workspace::routes as ws_routes;

fn cors_preflight() -> worker::Result<Response> {
    let mut resp = Response::empty()?;
    let headers = resp.headers_mut();
    let _ = headers.set("Access-Control-Allow-Origin", "*");
    let _ = headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, PATCH, DELETE, OPTIONS");
    let _ = headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization");
    let _ = headers.set("Access-Control-Max-Age", "86400");
    Ok(resp.with_status(204))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();

    if req.method() == Method::Options {
        return cors_preflight();
    }

    let router = Router::new();
    let router = auth_routes::register(router);
    let router = ws_routes::register(router);
    let router = tx_routes::register(router);
    let router = stmt_routes::register(router);
    let router = insight_routes::register(router);
    let router = docs_routes::register(router);
    let router = router.get_async("/", |_, _| async move { Response::ok("AI Finance Tracker API — docs at /api/docs") });

    router
        .run(req, env)
        .await
        .map(apply_security_headers)
}
