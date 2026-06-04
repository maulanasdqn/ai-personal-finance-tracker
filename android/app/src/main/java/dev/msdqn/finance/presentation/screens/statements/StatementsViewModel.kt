package dev.msdqn.finance.presentation.screens.statements

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.model.BankStatement
import dev.msdqn.finance.domain.repo.BankStatementRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class StatementsUiState(
    val isLoading: Boolean = false,
    val statements: List<BankStatement> = emptyList(),
    val error: String? = null,
)

@HiltViewModel
class StatementsViewModel @Inject constructor(
    private val repo: BankStatementRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(StatementsUiState())
    val uiState = _uiState.asStateFlow()

    fun load() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first().takeIf { !it.isNullOrBlank() } ?: run {
                _uiState.update { it.copy(isLoading = false, error = "Pilih workspace terlebih dahulu") }
                return@launch
            }
            repo.listStatements(token, workspaceId)
                .onSuccess { list -> _uiState.update { it.copy(statements = list, isLoading = false) } }
                .onFailure { e -> _uiState.update { it.copy(error = e.message, isLoading = false) } }
        }
    }
}
