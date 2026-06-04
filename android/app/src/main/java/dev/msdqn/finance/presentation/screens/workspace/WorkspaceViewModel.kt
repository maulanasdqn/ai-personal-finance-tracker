package dev.msdqn.finance.presentation.screens.workspace

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.model.Workspace
import dev.msdqn.finance.domain.repo.WorkspaceRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import javax.inject.Inject

data class WorkspaceUiState(
    val isLoading: Boolean = false,
    val workspaces: List<Workspace> = emptyList(),
    val error: String? = null,
)

@HiltViewModel
class WorkspaceViewModel @Inject constructor(
    private val wsRepo: WorkspaceRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(WorkspaceUiState())
    val uiState = _uiState.asStateFlow()

    fun load() {
        viewModelScope.launch {
            _uiState.value = WorkspaceUiState(isLoading = true)
            val token = session.token.first() ?: return@launch
            wsRepo.listWorkspaces(token)
                .onSuccess { _uiState.value = WorkspaceUiState(workspaces = it) }
                .onFailure { _uiState.value = WorkspaceUiState(error = it.message) }
        }
    }

    fun selectWorkspace(id: String) {
        viewModelScope.launch { session.saveWorkspaceId(id) }
    }
}
