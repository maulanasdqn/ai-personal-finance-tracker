package dev.msdqn.finance.domain.repo

import dev.msdqn.finance.domain.model.PaginationMeta
import dev.msdqn.finance.domain.model.Transaction

interface TransactionRepository {
    suspend fun listTransactions(
        token: String,
        workspaceId: String,
        category: String? = null,
        type: String? = null,
        from: String? = null,
        to: String? = null,
        limit: Int? = null,
        offset: Int? = null,
    ): Result<Pair<List<Transaction>, PaginationMeta>>

    suspend fun createTransaction(
        token: String,
        workspaceId: String,
        amount: Double,
        currency: String,
        category: String,
        description: String?,
        date: String,
        type: String,
    ): Result<Transaction>

    suspend fun deleteTransaction(token: String, workspaceId: String, id: String): Result<Unit>
}
