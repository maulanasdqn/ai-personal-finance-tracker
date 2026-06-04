package dev.msdqn.finance.presentation.screens.transaction

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.repo.TransactionRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import javax.inject.Inject

data class CreateTransactionState(
    val isLoading: Boolean = false,
    val success: Boolean = false,
    val error: String? = null,
)

@HiltViewModel
class CreateTransactionViewModel @Inject constructor(
    private val txRepo: TransactionRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _state = MutableStateFlow(CreateTransactionState())
    val state = _state.asStateFlow()

    fun create(amount: Double, currency: String, category: String, description: String?, date: String, type: String) {
        viewModelScope.launch {
            _state.value = CreateTransactionState(isLoading = true)
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first() ?: return@launch
            txRepo.createTransaction(token, workspaceId, amount, currency, category, description, date, type)
                .onSuccess { _state.value = CreateTransactionState(success = true) }
                .onFailure { _state.value = CreateTransactionState(error = it.message) }
        }
    }
}
