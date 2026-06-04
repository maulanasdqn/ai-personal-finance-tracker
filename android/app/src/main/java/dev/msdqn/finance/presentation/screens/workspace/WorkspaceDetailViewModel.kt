package dev.msdqn.finance.presentation.screens.workspace

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.model.Workspace
import dev.msdqn.finance.domain.model.WorkspaceMember
import dev.msdqn.finance.domain.repo.WorkspaceRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class WorkspaceDetailUiState(
    val workspace: Workspace? = null,
    val members: List<WorkspaceMember> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null,
    val inviteEmail: String = "",
)

@HiltViewModel
class WorkspaceDetailViewModel @Inject constructor(
    private val workspaceRepo: WorkspaceRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(WorkspaceDetailUiState())
    val uiState = _uiState.asStateFlow()

    fun load(workspaceId: String) {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            val token = session.token.first() ?: return@launch
            workspaceRepo.getWorkspace(token, workspaceId)
                .onSuccess { ws -> _uiState.update { it.copy(workspace = ws) } }
                .onFailure { e -> _uiState.update { it.copy(error = e.message, isLoading = false) } }
            workspaceRepo.listMembers(token, workspaceId)
                .onSuccess { members -> _uiState.update { it.copy(members = members, isLoading = false) } }
                .onFailure { e -> _uiState.update { it.copy(error = e.message, isLoading = false) } }
        }
    }

    fun setInviteEmail(email: String) {
        _uiState.update { it.copy(inviteEmail = email) }
    }

    fun invite(workspaceId: String) {
        val email = _uiState.value.inviteEmail.trim()
        if (email.isBlank()) return
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            val token = session.token.first() ?: return@launch
            workspaceRepo.addMember(token, workspaceId, email)
                .onSuccess {
                    _uiState.update { it.copy(inviteEmail = "") }
                    load(workspaceId)
                }
                .onFailure { e -> _uiState.update { it.copy(error = e.message, isLoading = false) } }
        }
    }

    fun selectWorkspace(workspaceId: String) {
        viewModelScope.launch { session.saveWorkspaceId(workspaceId) }
    }
}
