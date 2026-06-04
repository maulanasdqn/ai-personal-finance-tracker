package dev.msdqn.finance.domain.model

data class AiInsight(
    val id: String,
    val workspaceId: String,
    val insightType: String,
    val content: String,
    val dateFrom: String,
    val dateTo: String,
    val createdAt: String,
)
