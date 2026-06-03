use super::dto::InviteMemberInput;
use crate::domain::workspace::entity::{MemberRole, WorkspaceMember};
use crate::error::AppError;
use crate::infrastructure::repository::{user::D1UserRepository, workspace::D1WorkspaceRepository};

pub async fn execute(input: InviteMemberInput, workspace_repo: &D1WorkspaceRepository, user_repo: &D1UserRepository) -> Result<WorkspaceMember, AppError> {
    let member = workspace_repo.find_member(&input.workspace_id, &input.inviter_id).await?
        .ok_or_else(|| AppError::Forbidden("not a workspace member".into()))?;
    if member.role == MemberRole::Member {
        return Err(AppError::Forbidden("only owners and admins can invite".into()));
    }
    let user = user_repo.find_by_email(&input.email).await?
        .ok_or_else(|| AppError::NotFound("no account found with that email address".into()))?;
    let now = chrono::Utc::now().to_rfc3339();
    workspace_repo.add_member(&input.workspace_id, &user.id, MemberRole::Member, &now).await
}
