use crate::domain::workspace::entity::{MemberRole, NewWorkspace, Workspace, WorkspaceMember, WorkspacePatch};
use crate::error::AppError;

pub trait WorkspaceRepository {
    async fn create(&self, workspace: NewWorkspace) -> Result<Workspace, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Workspace>, AppError>;
    async fn list_by_user(&self, user_id: &str) -> Result<Vec<Workspace>, AppError>;
    async fn update(&self, id: &str, patch: WorkspacePatch, updated_at: &str) -> Result<Workspace, AppError>;
    async fn delete(&self, id: &str) -> Result<(), AppError>;
    async fn add_member(&self, workspace_id: &str, user_id: &str, role: MemberRole, now: &str) -> Result<WorkspaceMember, AppError>;
    async fn find_member(&self, workspace_id: &str, user_id: &str) -> Result<Option<WorkspaceMember>, AppError>;
    async fn list_members(&self, workspace_id: &str) -> Result<Vec<WorkspaceMember>, AppError>;
}
