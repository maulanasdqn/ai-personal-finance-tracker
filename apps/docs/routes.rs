use crate::domain::common::routes::DocsRoutes;
use super::handlers;
use worker::Router;

pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
    router
        .get_async(DocsRoutes::SPEC, |r, c| async move { handlers::spec_handler(r, c).await })
        .get_async(DocsRoutes::UI, |r, c| async move { handlers::ui_handler(r, c).await })
}
