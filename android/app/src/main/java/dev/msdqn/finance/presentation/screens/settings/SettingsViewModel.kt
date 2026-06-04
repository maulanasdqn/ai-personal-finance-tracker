package dev.msdqn.finance.presentation.screens.settings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class SettingsViewModel @Inject constructor(private val session: SessionManager) : ViewModel() {

    val email = session.email.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)
    val fullName = session.fullName.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    fun logout(onDone: () -> Unit) {
        viewModelScope.launch {
            session.clearSession()
            onDone()
        }
    }
}
