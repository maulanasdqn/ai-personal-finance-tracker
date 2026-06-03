use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}

#[derive(Deserialize)]
pub struct InviteMemberRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct MemberResponse {
    pub workspace_id: String,
    pub user_id: String,
    pub role: String,
    pub joined_at: String,
}

impl From<crate::domain::workspace::entity::Workspace> for WorkspaceResponse {
    fn from(w: crate::domain::workspace::entity::Workspace) -> Self {
        Self { id: w.id, name: w.name, description: w.description, owner_id: w.owner_id, created_at: w.created_at, updated_at: w.updated_at }
    }
}

impl From<crate::domain::workspace::entity::WorkspaceMember> for MemberResponse {
    fn from(m: crate::domain::workspace::entity::WorkspaceMember) -> Self {
        Self { workspace_id: m.workspace_id, user_id: m.user_id, role: m.role.to_string(), joined_at: m.joined_at }
    }
}
