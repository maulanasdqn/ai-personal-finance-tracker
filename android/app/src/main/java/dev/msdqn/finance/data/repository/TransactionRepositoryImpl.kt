package dev.msdqn.finance.data.repository

import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.data.remote.FinanceSdkSource
import dev.msdqn.finance.domain.model.PaginationMeta
import dev.msdqn.finance.domain.model.Transaction
import dev.msdqn.finance.domain.repo.TransactionRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.withContext
import javax.inject.Inject

class TransactionRepositoryImpl @Inject constructor(
    private val source: FinanceSdkSource,
    private val session: SessionManager,
) : TransactionRepository {

    private suspend fun base() = session.baseUrl.first()

    override suspend fun listTransactions(
        token: String, workspaceId: String, category: String?, type: String?,
        from: String?, to: String?, limit: Int?, offset: Int?,
    ): Result<Pair<List<Transaction>, PaginationMeta>> = withContext(Dispatchers.IO) {
        source.listTransactions(base(), token, workspaceId, category, type, from, to, limit, offset).mapCatching { json ->
            val arr = json.getJSONArray("data")
            val txs = (0 until arr.length()).map { i ->
                val o = arr.getJSONObject(i)
                Transaction(o.getString("id"), o.getString("workspace_id"), o.getDouble("amount"), o.getString("currency"), o.getString("category"), o.optString("description").takeIf { it.isNotEmpty() }, o.getString("transaction_date"), o.getString("transaction_type"), o.getString("created_by"), o.getString("created_at"))
            }
            val m = json.getJSONObject("meta")
            val meta = PaginationMeta(m.getLong("page"), m.getLong("per_page"), m.getLong("total_page"), m.getLong("total_data"))
            Pair(txs, meta)
        }
    }

    override suspend fun createTransaction(
        token: String, workspaceId: String, amount: Double, currency: String,
        category: String, description: String?, date: String, type: String,
    ): Result<Transaction> = withContext(Dispatchers.IO) {
        source.createTransaction(base(), token, workspaceId, amount, currency, category, description, date, type).mapCatching { json ->
            val o = json.getJSONObject("data")
            Transaction(o.getString("id"), o.getString("workspace_id"), o.getDouble("amount"), o.getString("currency"), o.getString("category"), o.optString("description").takeIf { it.isNotEmpty() }, o.getString("transaction_date"), o.getString("transaction_type"), o.getString("created_by"), o.getString("created_at"))
        }
    }

    override suspend fun deleteTransaction(token: String, workspaceId: String, id: String): Result<Unit> = withContext(Dispatchers.IO) {
        source.deleteTransaction(base(), token, workspaceId, id).map { }
    }
}
