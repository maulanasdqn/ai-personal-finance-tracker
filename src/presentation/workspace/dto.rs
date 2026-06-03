use serde::{Deserialize, Serialize};
use zod_rs::prelude::*;

#[derive(Deserialize, ZodSchema)]
pub struct CreateWorkspaceRequest {
    #[zod(min_length(1), max_length(80))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}

#[derive(Deserialize, ZodSchema)]
pub struct InviteMemberRequest {
    #[zod(email)]
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    #[wasm_bindgen_test]
    fn create_workspace_valid() {
        let v = serde_json::json!({"name": "My Workspace"});
        assert!(CreateWorkspaceRequest::validate_and_parse(&v).is_ok());
    }

    #[wasm_bindgen_test]
    fn create_workspace_empty_name() {
        let v = serde_json::json!({"name": ""});
        assert!(CreateWorkspaceRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn create_workspace_name_over_80_chars() {
        let v = serde_json::json!({"name": "a".repeat(81)});
        assert!(CreateWorkspaceRequest::validate_and_parse(&v).is_err());
    }

    #[wasm_bindgen_test]
    fn invite_member_valid_email() {
        let v = serde_json::json!({"email": "member@example.com"});
        assert!(InviteMemberRequest::validate_and_parse(&v).is_ok());
    }

    #[wasm_bindgen_test]
    fn invite_member_invalid_email() {
        let v = serde_json::json!({"email": "not-valid"});
        assert!(InviteMemberRequest::validate_and_parse(&v).is_err());
    }
}
