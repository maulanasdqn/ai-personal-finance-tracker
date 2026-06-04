package dev.msdqn.finance.data.repository

import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.data.remote.FinanceSdkSource
import dev.msdqn.finance.domain.model.Workspace
import dev.msdqn.finance.domain.model.WorkspaceMember
import dev.msdqn.finance.domain.repo.WorkspaceRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.withContext
import javax.inject.Inject

class WorkspaceRepositoryImpl @Inject constructor(
    private val source: FinanceSdkSource,
    private val session: SessionManager,
) : WorkspaceRepository {

    private suspend fun base() = session.baseUrl.first()

    override suspend fun listWorkspaces(token: String): Result<List<Workspace>> = withContext(Dispatchers.IO) {
        source.listWorkspaces(base(), token).mapCatching { json ->
            val arr = json.getJSONArray("data")
            (0 until arr.length()).map { i ->
                val o = arr.getJSONObject(i)
                Workspace(o.getString("id"), o.getString("name"), o.optString("description").takeIf { it.isNotEmpty() }, o.getString("owner_id"), o.getString("created_at"))
            }
        }
    }

    override suspend fun getWorkspace(token: String, id: String): Result<Workspace> = withContext(Dispatchers.IO) {
        source.getWorkspace(base(), token, id).mapCatching { json ->
            val o = json.getJSONObject("data")
            Workspace(o.getString("id"), o.getString("name"), o.optString("description").takeIf { it.isNotEmpty() }, o.getString("owner_id"), o.getString("created_at"))
        }
    }

    override suspend fun createWorkspace(token: String, name: String, description: String?): Result<Workspace> = withContext(Dispatchers.IO) {
        source.createWorkspace(base(), token, name, description).mapCatching { json ->
            val o = json.getJSONObject("data")
            Workspace(o.getString("id"), o.getString("name"), o.optString("description").takeIf { it.isNotEmpty() }, o.getString("owner_id"), o.getString("created_at"))
        }
    }

    override suspend fun updateWorkspace(token: String, id: String, name: String?, description: String?): Result<Workspace> = withContext(Dispatchers.IO) {
        source.updateWorkspace(base(), token, id, name, description).mapCatching { json ->
            val o = json.getJSONObject("data")
            Workspace(o.getString("id"), o.getString("name"), o.optString("description").takeIf { it.isNotEmpty() }, o.getString("owner_id"), o.getString("created_at"))
        }
    }

    override suspend fun deleteWorkspace(token: String, id: String): Result<Unit> = withContext(Dispatchers.IO) {
        source.deleteWorkspace(base(), token, id).map { }
    }

    override suspend fun listMembers(token: String, workspaceId: String): Result<List<WorkspaceMember>> = withContext(Dispatchers.IO) {
        source.listMembers(base(), token, workspaceId).mapCatching { json ->
            val arr = json.getJSONArray("data")
            (0 until arr.length()).map { i ->
                val o = arr.getJSONObject(i)
                WorkspaceMember(o.getString("id"), o.getString("workspace_id"), o.getString("user_id"), o.getString("email"), o.getString("full_name"), o.getString("role"), o.getString("joined_at"))
            }
        }
    }

    override suspend fun addMember(token: String, workspaceId: String, email: String): Result<WorkspaceMember> = withContext(Dispatchers.IO) {
        source.addMember(base(), token, workspaceId, email).mapCatching { json ->
            val o = json.getJSONObject("data")
            WorkspaceMember(o.getString("id"), o.getString("workspace_id"), o.getString("user_id"), o.getString("email"), o.getString("full_name"), o.getString("role"), o.getString("joined_at"))
        }
    }
}
