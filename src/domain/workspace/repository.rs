use crate::domain::workspace::entity::{MemberRole, NewWorkspace, Workspace, WorkspaceMember, WorkspacePatch};
use crate::error::AppError;
use std::future::Future;

pub trait WorkspaceRepository: Send + Sync {
    fn create(&self, workspace: NewWorkspace) -> impl Future<Output = Result<Workspace, AppError>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<Option<Workspace>, AppError>> + Send;
    fn list_by_user(&self, user_id: &str) -> impl Future<Output = Result<Vec<Workspace>, AppError>> + Send;
    fn update(&self, id: &str, patch: WorkspacePatch) -> impl Future<Output = Result<Workspace, AppError>> + Send;
    fn delete(&self, id: &str) -> impl Future<Output = Result<(), AppError>> + Send;
    fn add_member(&self, workspace_id: &str, user_id: &str, role: MemberRole) -> impl Future<Output = Result<WorkspaceMember, AppError>> + Send;
    fn remove_member(&self, workspace_id: &str, user_id: &str) -> impl Future<Output = Result<(), AppError>> + Send;
    fn find_member(&self, workspace_id: &str, user_id: &str) -> impl Future<Output = Result<Option<WorkspaceMember>, AppError>> + Send;
    fn list_members(&self, workspace_id: &str) -> impl Future<Output = Result<Vec<WorkspaceMember>, AppError>> + Send;
}
