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
    val recentTransactions: List<Transaction> = emptyList(),
    val totalBalance: Double = 0.0,
    val totalIncome: Double = 0.0,
    val totalExpense: Double = 0.0,
    val todayChange: Double = 0.0,
    val currency: String = "IDR",
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
            val workspaceId = session.workspaceId.first().takeIf { !it.isNullOrBlank() } ?: return@launch
            txRepo.listTransactions(token, workspaceId, limit = 100)
                .onSuccess { (txs, _) ->
                    val income = txs.filter { it.transactionType == "income" }.sumOf { it.amount }
                    val expense = txs.filter { it.transactionType == "expense" }.sumOf { it.amount }
                    val balance = income - expense
                    val today = java.time.LocalDate.now().toString()
                    val todayIncome = txs.filter { it.transactionType == "income" && it.transactionDate == today }.sumOf { it.amount }
                    val todayExpense = txs.filter { it.transactionType == "expense" && it.transactionDate == today }.sumOf { it.amount }
                    val currency = txs.firstOrNull()?.currency ?: "USD"
                    _uiState.value = HomeUiState(
                        recentTransactions = txs.take(5),
                        totalBalance = balance,
                        totalIncome = income,
                        totalExpense = expense,
                        todayChange = todayIncome - todayExpense,
                        currency = currency,
                    )
                }
                .onFailure { _uiState.value = HomeUiState(error = it.message) }
        }
    }
}
