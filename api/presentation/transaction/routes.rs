use super::handlers;
use crate::domain::common::routes::TransactionRoutes;
use worker::Router;

pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
    router
        .get_async(TransactionRoutes::BASE, |r, c| async move {
            handlers::list_handler(r, c).await
        })
        .post_async(TransactionRoutes::BASE, |r, c| async move {
            handlers::create_handler(r, c).await
        })
        .get_async(TransactionRoutes::BY_ID, |r, c| async move {
            handlers::get_handler(r, c).await
        })
        .delete_async(TransactionRoutes::BY_ID, |r, c| async move {
            handlers::delete_handler(r, c).await
        })
}
