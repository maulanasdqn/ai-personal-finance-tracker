package dev.msdqn.finance.presentation.screens.auth

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.repo.AuthRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

data class AuthUiState(
    val isLoading: Boolean = false,
    val error: String? = null,
)

@HiltViewModel
class AuthViewModel @Inject constructor(
    private val authRepo: AuthRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(AuthUiState())
    val uiState = _uiState.asStateFlow()

    fun login(email: String, password: String, onSuccess: () -> Unit) {
        viewModelScope.launch {
            _uiState.value = AuthUiState(isLoading = true)
            authRepo.login(email, password)
                .onSuccess { onSuccess() }
                .onFailure { _uiState.value = AuthUiState(error = it.message) }
        }
    }

    fun register(email: String, password: String, fullName: String, onSuccess: () -> Unit) {
        viewModelScope.launch {
            _uiState.value = AuthUiState(isLoading = true)
            authRepo.register(email, password, fullName)
                .onSuccess { onSuccess() }
                .onFailure { _uiState.value = AuthUiState(error = it.message) }
        }
    }

    fun saveBaseUrl(url: String) {
        viewModelScope.launch { session.saveBaseUrl(url) }
    }

    fun clearError() {
        _uiState.value = _uiState.value.copy(error = null)
    }
}
