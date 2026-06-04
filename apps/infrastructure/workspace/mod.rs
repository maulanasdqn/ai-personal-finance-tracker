use crate::domain::workspace::entity::{MemberRole, NewWorkspace, Workspace, WorkspaceMember, WorkspacePatch};
use crate::domain::workspace::repository::WorkspaceRepository;
use crate::error::AppError;
use rust_d1_orm::{opt_js, D1Model, Query, Set, Table};
use serde::Deserialize;
use std::str::FromStr;
use worker::{wasm_bindgen::JsValue, D1Database};

#[derive(Deserialize)]
struct WorkspaceRow {
    id: String,
    name: String,
    description: Option<String>,
    owner_id: String,
    created_at: String,
    updated_at: String,
}

impl D1Model for WorkspaceRow {
    const TABLE: &'static str = "workspaces";
    const COLUMNS: &'static [&'static str] = &["id", "name", "description", "owner_id", "created_at", "updated_at"];
    fn values(&self) -> Vec<JsValue> {
        vec![
            self.id.clone().into(), self.name.clone().into(),
            opt_js(self.description.clone()), self.owner_id.clone().into(),
            self.created_at.clone().into(), self.updated_at.clone().into(),
        ]
    }
}

impl From<WorkspaceRow> for Workspace {
    fn from(r: WorkspaceRow) -> Self {
        Self { id: r.id, name: r.name, description: r.description, owner_id: r.owner_id, created_at: r.created_at, updated_at: r.updated_at }
    }
}

#[derive(Deserialize)]
struct MemberRow {
    workspace_id: String,
    user_id: String,
    role: String,
    joined_at: String,
}

impl D1Model for MemberRow {
    const TABLE: &'static str = "workspace_members";
    const COLUMNS: &'static [&'static str] = &["workspace_id", "user_id", "role", "joined_at"];
    fn values(&self) -> Vec<JsValue> {
        vec![
            self.workspace_id.clone().into(), self.user_id.clone().into(),
            self.role.clone().into(), self.joined_at.clone().into(),
        ]
    }
}

impl From<MemberRow> for WorkspaceMember {
    fn from(r: MemberRow) -> Self {
        Self {
            workspace_id: r.workspace_id, user_id: r.user_id,
            role: MemberRole::from_str(&r.role).unwrap_or(MemberRole::Member),
            joined_at: r.joined_at,
        }
    }
}

pub struct D1WorkspaceRepository { db: D1Database }

impl D1WorkspaceRepository {
    pub fn new(db: D1Database) -> Self { Self { db } }
}

impl WorkspaceRepository for D1WorkspaceRepository {
    async fn create(&self, w: NewWorkspace) -> Result<Workspace, AppError> {
        let row = WorkspaceRow {
            id: w.id, name: w.name, description: w.description,
            owner_id: w.owner_id, created_at: w.created_at, updated_at: w.updated_at,
        };
        Table::<WorkspaceRow>::new(&self.db)
            .insert(&row).await
            .map_err(|_| AppError::Internal)
            .map(Into::into)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Workspace>, AppError> {
        Table::<WorkspaceRow>::new(&self.db)
            .find_one(Query::new().eq("id", id)).await
            .map_err(|_| AppError::Internal)
            .map(|r| r.map(Into::into))
    }

    async fn list_by_user(&self, user_id: &str) -> Result<Vec<Workspace>, AppError> {
        // JOIN query — not expressible with single-table builder; drop to raw D1 via explicit columns
        let cols = WorkspaceRow::COLUMNS.iter().map(|c| format!("w.{}", c)).collect::<Vec<_>>().join(", ");
        let sql = format!("SELECT {} FROM workspaces w JOIN workspace_members wm ON w.id = wm.workspace_id WHERE wm.user_id = ?1 ORDER BY w.created_at DESC", cols);
        let result = self.db.prepare(&sql)
            .bind(&[user_id.into()])
            .map_err(|_| AppError::Internal)?
            .all().await.map_err(|_| AppError::Internal)?;
        result.results::<WorkspaceRow>()
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    async fn update(&self, id: &str, patch: WorkspacePatch, now: &str) -> Result<Workspace, AppError> {
        let mut set = Set::new();
        if let Some(name) = patch.name { set = set.field("name", name); }
        if let Some(desc) = patch.description { set = set.nullable_field("description", desc); }
        if set.is_empty() {
            return self.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("workspace not found".into()));
        }
        set = set.field("updated_at", now);
        Table::<WorkspaceRow>::new(&self.db)
            .update(set, Query::new().eq("id", id)).await
            .map_err(|_| AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("workspace not found".into()))
            .map(Into::into)
    }

    async fn delete(&self, id: &str) -> Result<(), AppError> {
        Table::<WorkspaceRow>::new(&self.db)
            .delete(Query::new().eq("id", id)).await
            .map_err(|_| AppError::Internal)
    }

    async fn add_member(&self, workspace_id: &str, user_id: &str, role: MemberRole, now: &str) -> Result<WorkspaceMember, AppError> {
        if self.find_member(workspace_id, user_id).await?.is_some() {
            return Err(AppError::Conflict("user is already a member of this workspace".into()));
        }
        let row = MemberRow {
            workspace_id: workspace_id.to_string(), user_id: user_id.to_string(),
            role: role.to_string(), joined_at: now.to_string(),
        };
        Table::<MemberRow>::new(&self.db)
            .insert(&row).await
            .map_err(|_| AppError::Internal)
            .map(Into::into)
    }

    async fn find_member(&self, workspace_id: &str, user_id: &str) -> Result<Option<WorkspaceMember>, AppError> {
        Table::<MemberRow>::new(&self.db)
            .find_one(Query::new().eq("workspace_id", workspace_id).eq("user_id", user_id)).await
            .map_err(|_| AppError::Internal)
            .map(|r| r.map(Into::into))
    }

    async fn list_members(&self, workspace_id: &str) -> Result<Vec<WorkspaceMember>, AppError> {
        Table::<MemberRow>::new(&self.db)
            .find_all(Query::new().eq("workspace_id", workspace_id)).await
            .map_err(|_| AppError::Internal)
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }
}
