use crate::domain::common::routes::WorkspaceRoutes;
use super::handlers;
use worker::Router;

pub fn register(router: Router<'static, ()>) -> Router<'static, ()> {
    router
        .get_async(WorkspaceRoutes::BASE, |r, c| async move { handlers::list_handler(r, c).await })
        .post_async(WorkspaceRoutes::BASE, |r, c| async move { handlers::create_handler(r, c).await })
        .get_async(WorkspaceRoutes::BY_ID, |r, c| async move { handlers::get_handler(r, c).await })
        .put_async(WorkspaceRoutes::BY_ID, |r, c| async move { handlers::update_handler(r, c).await })
        .delete_async(WorkspaceRoutes::BY_ID, |r, c| async move { handlers::delete_handler(r, c).await })
        .post_async(WorkspaceRoutes::MEMBERS, |r, c| async move { handlers::invite_handler(r, c).await })
        .get_async(WorkspaceRoutes::MEMBERS, |r, c| async move { handlers::members_handler(r, c).await })
}
