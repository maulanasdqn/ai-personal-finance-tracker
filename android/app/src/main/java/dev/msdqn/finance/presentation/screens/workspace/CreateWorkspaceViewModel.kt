package dev.msdqn.finance.presentation.screens.workspace

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.repo.WorkspaceRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import javax.inject.Inject

data class CreateWorkspaceState(val isLoading: Boolean = false, val success: Boolean = false, val error: String? = null)

@HiltViewModel
class CreateWorkspaceViewModel @Inject constructor(
    private val wsRepo: WorkspaceRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _state = MutableStateFlow(CreateWorkspaceState())
    val state = _state.asStateFlow()

    fun create(name: String, description: String?) {
        viewModelScope.launch {
            _state.value = CreateWorkspaceState(isLoading = true)
            val token = session.token.first() ?: return@launch
            wsRepo.createWorkspace(token, name, description)
                .onSuccess { _state.value = CreateWorkspaceState(success = true) }
                .onFailure { _state.value = CreateWorkspaceState(error = it.message) }
        }
    }
}
