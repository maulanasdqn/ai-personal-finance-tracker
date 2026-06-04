use super::dto::CreateWorkspaceInput;
use crate::domain::workspace::entity::{MemberRole, NewWorkspace};
use crate::domain::workspace::repository::WorkspaceRepository;
use crate::error::AppError;
use uuid::Uuid;

pub async fn execute(input: CreateWorkspaceInput, repo: &impl WorkspaceRepository) -> Result<crate::domain::workspace::entity::Workspace, AppError> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let workspace = repo.create(NewWorkspace {
        id: id.clone(),
        name: input.name,
        description: input.description,
        owner_id: input.owner_id.clone(),
        created_at: now.clone(),
        updated_at: now.clone(),
    }).await?;
    repo.add_member(&id, &input.owner_id, MemberRole::Owner, &now).await?;
    Ok(workspace)
}
