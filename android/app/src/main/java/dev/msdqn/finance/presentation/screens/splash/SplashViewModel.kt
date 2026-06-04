package dev.msdqn.finance.presentation.screens.splash

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.stateIn
import javax.inject.Inject

@HiltViewModel
class SplashViewModel @Inject constructor(session: SessionManager) : ViewModel() {

    val isAuthenticated = combine(session.token, session.baseUrl) { token, baseUrl ->
        token != null && baseUrl != null
    }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)
}
