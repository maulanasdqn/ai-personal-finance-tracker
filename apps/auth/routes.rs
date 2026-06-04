use crate::domain::common::routes::AuthRoutes;
use super::handlers;
use worker::Router;

pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
    router
        .post_async(AuthRoutes::REGISTER, |r, c| async move { handlers::register_handler(r, c).await })
        .post_async(AuthRoutes::LOGIN, |r, c| async move { handlers::login_handler(r, c).await })
}
