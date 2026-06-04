package dev.msdqn.finance.domain.model

data class Transaction(
    val id: String,
    val workspaceId: String,
    val amount: Double,
    val currency: String,
    val category: String,
    val description: String?,
    val transactionDate: String,
    val transactionType: String,
    val createdBy: String,
    val createdAt: String,
)

data class PaginationMeta(
    val page: Long,
    val perPage: Long,
    val totalPage: Long,
    val totalData: Long,
)
