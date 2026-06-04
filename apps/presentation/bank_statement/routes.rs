use super::handlers;
use crate::domain::common::routes::BankStatementRoutes;
use worker::Router;

pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
    router
        .get_async(BankStatementRoutes::BASE, |r, c| async move {
            handlers::list_handler(r, c).await
        })
        .post_async(BankStatementRoutes::BASE, |r, c| async move {
            handlers::upload_handler(r, c).await
        })
        .get_async(BankStatementRoutes::BY_ID, |r, c| async move {
            handlers::get_handler(r, c).await
        })
}
