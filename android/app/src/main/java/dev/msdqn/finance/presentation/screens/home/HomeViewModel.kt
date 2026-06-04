package dev.msdqn.finance.presentation.screens.home

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.model.Transaction
import dev.msdqn.finance.domain.repo.TransactionRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import javax.inject.Inject

data class HomeUiState(
    val isLoading: Boolean = false,
    val fullName: String = "",
    val recentTransactions: List<Transaction> = emptyList(),
    val totalIncome: Double = 0.0,
    val totalExpense: Double = 0.0,
    val error: String? = null,
)

@HiltViewModel
class HomeViewModel @Inject constructor(
    private val txRepo: TransactionRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(HomeUiState())
    val uiState = _uiState.asStateFlow()

    val fullName = session.fullName.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), "")

    fun load() {
        viewModelScope.launch {
            _uiState.value = HomeUiState(isLoading = true)
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first() ?: return@launch
            txRepo.listTransactions(token, workspaceId, limit = 5)
                .onSuccess { (txs, _) ->
                    val income = txs.filter { it.transactionType == "income" }.sumOf { it.amount }
                    val expense = txs.filter { it.transactionType == "expense" }.sumOf { it.amount }
                    _uiState.value = HomeUiState(recentTransactions = txs, totalIncome = income, totalExpense = expense)
                }
                .onFailure { _uiState.value = HomeUiState(error = it.message) }
        }
    }
}
