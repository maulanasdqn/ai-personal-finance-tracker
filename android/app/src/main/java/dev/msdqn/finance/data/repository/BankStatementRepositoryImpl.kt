package dev.msdqn.finance.data.repository

import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.data.remote.FinanceSdkSource
import dev.msdqn.finance.domain.model.BankStatement
import dev.msdqn.finance.domain.repo.BankStatementRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.withContext
import javax.inject.Inject

class BankStatementRepositoryImpl @Inject constructor(
    private val source: FinanceSdkSource,
    private val session: SessionManager,
) : BankStatementRepository {

    private suspend fun base() = session.baseUrl.first()

    override suspend fun listStatements(token: String, workspaceId: String): Result<List<BankStatement>> = withContext(Dispatchers.IO) {
        source.listStatements(base(), token, workspaceId).mapCatching { json ->
            val arr = json.getJSONArray("data")
            (0 until arr.length()).map { i ->
                val o = arr.getJSONObject(i)
                BankStatement(
                    id = o.getString("id"),
                    workspaceId = o.getString("workspace_id"),
                    fileName = o.getString("file_name"),
                    fileUrl = o.getString("file_url"),
                    uploadedAt = o.getString("uploaded_at"),
                )
            }
        }
    }

    override suspend fun getStatement(token: String, workspaceId: String, id: String): Result<BankStatement> = withContext(Dispatchers.IO) {
        source.getStatement(base(), token, workspaceId, id).mapCatching { json ->
            val o = json.getJSONObject("data")
            BankStatement(
                id = o.getString("id"),
                workspaceId = o.getString("workspace_id"),
                fileName = o.getString("file_name"),
                fileUrl = o.getString("file_url"),
                uploadedAt = o.getString("uploaded_at"),
            )
        }
    }
}
