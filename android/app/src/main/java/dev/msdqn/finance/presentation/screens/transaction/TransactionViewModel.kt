package dev.msdqn.finance.presentation.screens.transaction

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.model.PaginationMeta
import dev.msdqn.finance.domain.model.Transaction
import dev.msdqn.finance.domain.repo.TransactionRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import javax.inject.Inject

data class TransactionUiState(
    val isLoading: Boolean = false,
    val transactions: List<Transaction> = emptyList(),
    val meta: PaginationMeta? = null,
    val error: String? = null,
)

@HiltViewModel
class TransactionViewModel @Inject constructor(
    private val txRepo: TransactionRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(TransactionUiState())
    val uiState = _uiState.asStateFlow()

    fun load() {
        viewModelScope.launch {
            _uiState.value = TransactionUiState(isLoading = true)
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first() ?: return@launch
            txRepo.listTransactions(token, workspaceId)
                .onSuccess { (txs, meta) -> _uiState.value = TransactionUiState(transactions = txs, meta = meta) }
                .onFailure { _uiState.value = TransactionUiState(error = it.message) }
        }
    }

    fun delete(id: String) {
        viewModelScope.launch {
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first() ?: return@launch
            txRepo.deleteTransaction(token, workspaceId, id).onSuccess { load() }
        }
    }
}
