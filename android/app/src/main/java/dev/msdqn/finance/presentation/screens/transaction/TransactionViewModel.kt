package dev.msdqn.finance.presentation.screens.transaction

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.model.Transaction
import dev.msdqn.finance.domain.repo.TransactionRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import java.time.LocalDate
import javax.inject.Inject

data class TransactionUiState(
    val isLoading: Boolean = false,
    val allTransactions: List<Transaction> = emptyList(),
    val grouped: List<Pair<String, List<Transaction>>> = emptyList(),
    val filter: String = "all",
    val error: String? = null,
)

private fun groupLabel(dateStr: String): String {
    return try {
        val date = LocalDate.parse(dateStr)
        val today = LocalDate.now()
        when {
            date == today -> "Hari Ini"
            date == today.minusDays(1) -> "Kemarin"
            else -> date.month.name.lowercase().replaceFirstChar { it.uppercase() } + " " + date.dayOfMonth
        }
    } catch (_: Exception) { dateStr }
}

private fun buildGrouped(txs: List<Transaction>, filter: String): List<Pair<String, List<Transaction>>> {
    val filtered = when (filter) {
        "income" -> txs.filter { it.transactionType == "income" }
        "expense" -> txs.filter { it.transactionType == "expense" }
        else -> txs
    }
    return filtered
        .groupBy { groupLabel(it.transactionDate) }
        .entries
        .sortedByDescending { it.value.first().transactionDate }
        .map { it.key to it.value }
}

@HiltViewModel
class TransactionViewModel @Inject constructor(
    private val txRepo: TransactionRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(TransactionUiState())
    val uiState = _uiState.asStateFlow()

    fun load() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first().takeIf { !it.isNullOrBlank() } ?: return@launch
            txRepo.listTransactions(token, workspaceId, limit = 100)
                .onSuccess { (txs, _) ->
                    val currentFilter = _uiState.value.filter
                    _uiState.update { it.copy(
                        isLoading = false,
                        allTransactions = txs,
                        grouped = buildGrouped(txs, currentFilter),
                    ) }
                }
                .onFailure { e -> _uiState.update { it.copy(isLoading = false, error = e.message) } }
        }
    }

    fun setFilter(filter: String) {
        _uiState.update { it.copy(
            filter = filter,
            grouped = buildGrouped(it.allTransactions, filter),
        ) }
    }

    fun delete(id: String) {
        viewModelScope.launch {
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first().takeIf { !it.isNullOrBlank() } ?: return@launch
            txRepo.deleteTransaction(token, workspaceId, id).onSuccess { load() }
        }
    }
}
