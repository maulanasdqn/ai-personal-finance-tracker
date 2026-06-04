#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::str_to_string,
    clippy::exit,
    clippy::mem_forget
)]
#![allow(
    clippy::future_not_send,      // WASM is single-threaded
    clippy::missing_errors_doc,   // no public doc comments
    clippy::implicit_clone,       // worker-rs types deref to String in some places
    clippy::missing_const_for_fn, // wasm-bindgen types are not const-compatible
    clippy::module_name_repetitions,
)]

use worker::{event, Context, Env, Method, Request, Response, Router};

mod application;
mod domain;
mod error;
mod infrastructure;
mod presentation;

use presentation::{
    ai_insight::routes as insight_routes, auth::routes as auth_routes,
    bank_statement::routes as stmt_routes, docs::routes as docs_routes,
    security::apply_security_headers, transaction::routes as tx_routes,
    workspace::routes as ws_routes,
};

fn cors_preflight() -> worker::Result<Response> {
    let mut resp = Response::empty()?;
    let headers = resp.headers_mut();
    let _ = headers.set("Access-Control-Allow-Origin", "*");
    let _ = headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    );
    let _ = headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization",
    );
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
    let router = router.get_async("/", |_, _| async move {
        Response::ok("AI Finance Tracker API — docs at /api/docs")
    });

    router.run(req, env).await.map(apply_security_headers)
}
