pub struct CreateWorkspaceInput {
    pub name: String,
    pub description: Option<String>,
    pub owner_id: String,
}

pub struct InviteMemberInput {
    pub workspace_id: String,
    pub inviter_id: String,
    pub email: String,
}
