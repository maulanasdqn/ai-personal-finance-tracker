package dev.msdqn.finance.data.repository

import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.data.remote.FinanceSdkSource
import dev.msdqn.finance.domain.model.AiInsight
import dev.msdqn.finance.domain.repo.InsightRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.withContext
import javax.inject.Inject

class InsightRepositoryImpl @Inject constructor(
    private val source: FinanceSdkSource,
    private val session: SessionManager,
) : InsightRepository {

    private suspend fun base() = session.baseUrl.first()

    override suspend fun listInsights(token: String, workspaceId: String, type: String?): Result<List<AiInsight>> = withContext(Dispatchers.IO) {
        source.listInsights(base(), token, workspaceId, type).mapCatching { json ->
            val arr = json.getJSONArray("data")
            (0 until arr.length()).map { i ->
                val o = arr.getJSONObject(i)
                AiInsight(o.getString("id"), o.getString("workspace_id"), o.getString("insight_type"), o.getString("content"), o.getString("date_from"), o.getString("date_to"), o.getString("created_at"))
            }
        }
    }

    override suspend fun generateInsights(token: String, workspaceId: String, dateFrom: String, dateTo: String): Result<List<AiInsight>> = withContext(Dispatchers.IO) {
        source.generateInsights(base(), token, workspaceId, dateFrom, dateTo).mapCatching { json ->
            val arr = json.getJSONArray("data")
            (0 until arr.length()).map { i ->
                val o = arr.getJSONObject(i)
                AiInsight(o.getString("id"), o.getString("workspace_id"), o.getString("insight_type"), o.getString("content"), o.getString("date_from"), o.getString("date_to"), o.getString("created_at"))
            }
        }
    }
}
