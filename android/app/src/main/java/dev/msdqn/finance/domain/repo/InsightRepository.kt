package dev.msdqn.finance.domain.repo

import dev.msdqn.finance.domain.model.AiInsight

interface InsightRepository {
    suspend fun listInsights(token: String, workspaceId: String, type: String? = null): Result<List<AiInsight>>
    suspend fun generateInsights(token: String, workspaceId: String, dateFrom: String, dateTo: String): Result<List<AiInsight>>
}
