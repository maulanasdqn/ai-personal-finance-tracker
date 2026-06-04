package dev.msdqn.finance.presentation.screens.insights

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.model.AiInsight
import dev.msdqn.finance.domain.repo.InsightRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class InsightsUiState(
    val isLoading: Boolean = false,
    val insights: List<AiInsight> = emptyList(),
    val error: String? = null,
)

@HiltViewModel
class InsightsViewModel @Inject constructor(
    private val insightRepo: InsightRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(InsightsUiState())
    val uiState = _uiState.asStateFlow()

    fun load() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first() ?: return@launch
            insightRepo.listInsights(token, workspaceId)
                .onSuccess { insights -> _uiState.update { it.copy(insights = insights, isLoading = false) } }
                .onFailure { e -> _uiState.update { it.copy(error = e.message, isLoading = false) } }
        }
    }

    fun generate(dateFrom: String, dateTo: String) {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first() ?: return@launch
            insightRepo.generateInsights(token, workspaceId, dateFrom, dateTo)
                .onSuccess { insights -> _uiState.update { it.copy(insights = it.insights + insights, isLoading = false) } }
                .onFailure { e -> _uiState.update { it.copy(error = e.message, isLoading = false) } }
        }
    }
}
