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
            _uiState.value = InsightsUiState(isLoading = true)
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first() ?: return@launch
            insightRepo.listInsights(token, workspaceId)
                .onSuccess { _uiState.value = InsightsUiState(insights = it) }
                .onFailure { _uiState.value = InsightsUiState(error = it.message) }
        }
    }
}
