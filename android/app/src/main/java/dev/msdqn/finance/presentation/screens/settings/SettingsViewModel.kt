package dev.msdqn.finance.presentation.screens.settings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.repo.WorkspaceRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class SettingsViewModel @Inject constructor(
    private val session: SessionManager,
    private val workspaceRepo: WorkspaceRepository,
) : ViewModel() {

    val email = session.email.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)
    val fullName = session.fullName.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    private val _workspaceName = MutableStateFlow<String?>(null)
    val workspaceName = _workspaceName.asStateFlow()

    init {
        viewModelScope.launch {
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first()
            if (workspaceId.isNullOrBlank()) return@launch
            workspaceRepo.getWorkspace(token, workspaceId)
                .onSuccess { ws -> _workspaceName.value = ws.name }
        }
    }

    fun logout(onDone: () -> Unit) {
        viewModelScope.launch {
            session.clearSession()
            onDone()
        }
    }
}
