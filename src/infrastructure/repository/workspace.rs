use crate::domain::workspace::entity::{MemberRole, NewWorkspace, Workspace, WorkspaceMember, WorkspacePatch};
use crate::error::AppError;
use serde::Deserialize;
use std::str::FromStr;
use worker::D1Database;

#[derive(Deserialize)]
struct WorkspaceRow {
    id: String,
    name: String,
    description: Option<String>,
    owner_id: String,
    created_at: String,
    updated_at: String,
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

impl From<MemberRow> for WorkspaceMember {
    fn from(r: MemberRow) -> Self {
        Self {
            workspace_id: r.workspace_id,
            user_id: r.user_id,
            role: MemberRole::from_str(&r.role).unwrap_or(MemberRole::Member),
            joined_at: r.joined_at,
        }
    }
}

pub struct D1WorkspaceRepository {
    db: D1Database,
}

impl D1WorkspaceRepository {
    pub fn new(db: D1Database) -> Self { Self { db } }

    pub async fn create(&self, w: NewWorkspace) -> Result<Workspace, AppError> {
        self.db
            .prepare("INSERT INTO workspaces (id, name, description, owner_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
            .bind(&[w.id.clone().into(), w.name.into(), w.description.into(), w.owner_id.into(), w.created_at.into(), w.updated_at.into()])
            .map_err(|e| AppError::Internal(e.to_string()))?
            .run().await.map_err(|e| AppError::Internal(e.to_string()))?;
        self.find_by_id(&w.id).await?.ok_or_else(|| AppError::Internal("insert failed".into()))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Workspace>, AppError> {
        self.db.prepare("SELECT * FROM workspaces WHERE id = ?1")
            .bind(&[id.into()]).map_err(|e| AppError::Internal(e.to_string()))?
            .first::<WorkspaceRow>(None).await
            .map_err(|e| AppError::Internal(e.to_string())).map(|r| r.map(Into::into))
    }

    pub async fn list_by_user(&self, user_id: &str) -> Result<Vec<Workspace>, AppError> {
        let results = self.db
            .prepare("SELECT w.* FROM workspaces w JOIN workspace_members wm ON w.id = wm.workspace_id WHERE wm.user_id = ?1 ORDER BY w.created_at DESC")
            .bind(&[user_id.into()]).map_err(|e| AppError::Internal(e.to_string()))?
            .all().await.map_err(|e| AppError::Internal(e.to_string()))?;
        results.results::<WorkspaceRow>()
            .map_err(|e| AppError::Internal(e.to_string()))
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn update(&self, id: &str, patch: WorkspacePatch, now: &str) -> Result<Workspace, AppError> {
        if let Some(name) = patch.name {
            self.db.prepare("UPDATE workspaces SET name = ?1, updated_at = ?2 WHERE id = ?3")
                .bind(&[name.into(), now.into(), id.into()])
                .map_err(|e| AppError::Internal(e.to_string()))?
                .run().await.map_err(|e| AppError::Internal(e.to_string()))?;
        }
        self.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("workspace not found".into()))
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        self.db.prepare("DELETE FROM workspaces WHERE id = ?1")
            .bind(&[id.into()]).map_err(|e| AppError::Internal(e.to_string()))?
            .run().await.map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    pub async fn add_member(&self, workspace_id: &str, user_id: &str, role: MemberRole, now: &str) -> Result<WorkspaceMember, AppError> {
        self.db
            .prepare("INSERT OR IGNORE INTO workspace_members (workspace_id, user_id, role, joined_at) VALUES (?1, ?2, ?3, ?4)")
            .bind(&[workspace_id.into(), user_id.into(), role.to_string().into(), now.into()])
            .map_err(|e| AppError::Internal(e.to_string()))?
            .run().await.map_err(|e| AppError::Conflict(e.to_string()))?;
        self.find_member(workspace_id, user_id).await?.ok_or_else(|| AppError::Internal("insert failed".into()))
    }

    pub async fn remove_member(&self, workspace_id: &str, user_id: &str) -> Result<(), AppError> {
        self.db.prepare("DELETE FROM workspace_members WHERE workspace_id = ?1 AND user_id = ?2")
            .bind(&[workspace_id.into(), user_id.into()])
            .map_err(|e| AppError::Internal(e.to_string()))?
            .run().await.map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    pub async fn find_member(&self, workspace_id: &str, user_id: &str) -> Result<Option<WorkspaceMember>, AppError> {
        self.db.prepare("SELECT * FROM workspace_members WHERE workspace_id = ?1 AND user_id = ?2")
            .bind(&[workspace_id.into(), user_id.into()])
            .map_err(|e| AppError::Internal(e.to_string()))?
            .first::<MemberRow>(None).await
            .map_err(|e| AppError::Internal(e.to_string())).map(|r| r.map(Into::into))
    }

    pub async fn list_members(&self, workspace_id: &str) -> Result<Vec<WorkspaceMember>, AppError> {
        let results = self.db.prepare("SELECT * FROM workspace_members WHERE workspace_id = ?1")
            .bind(&[workspace_id.into()]).map_err(|e| AppError::Internal(e.to_string()))?
            .all().await.map_err(|e| AppError::Internal(e.to_string()))?;
        results.results::<MemberRow>()
            .map_err(|e| AppError::Internal(e.to_string()))
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }
}
