use super::handlers;
use crate::domain::common::routes::AiInsightRoutes;
use worker::Router;

pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
    router
        .get_async(AiInsightRoutes::BASE, |r, c| async move {
            handlers::list_handler(r, c).await
        })
        .post_async(AiInsightRoutes::GENERATE, |r, c| async move {
            handlers::generate_handler(r, c).await
        })
}
