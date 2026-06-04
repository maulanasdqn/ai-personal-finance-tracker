package dev.msdqn.finance.domain.repo

import dev.msdqn.finance.domain.model.Workspace
import dev.msdqn.finance.domain.model.WorkspaceMember

interface WorkspaceRepository {
    suspend fun listWorkspaces(token: String): Result<List<Workspace>>
    suspend fun getWorkspace(token: String, id: String): Result<Workspace>
    suspend fun createWorkspace(token: String, name: String, description: String?): Result<Workspace>
    suspend fun updateWorkspace(token: String, id: String, name: String?, description: String?): Result<Workspace>
    suspend fun deleteWorkspace(token: String, id: String): Result<Unit>
    suspend fun listMembers(token: String, workspaceId: String): Result<List<WorkspaceMember>>
    suspend fun addMember(token: String, workspaceId: String, email: String): Result<WorkspaceMember>
}
