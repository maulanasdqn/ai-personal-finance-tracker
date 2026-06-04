package dev.msdqn.finance.presentation.screens.transaction

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.domain.repo.TransactionRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import java.time.LocalDate
import javax.inject.Inject

data class CreateTransactionState(
    val cents: Long = 0L,
    val category: String = "",
    val description: String = "",
    val date: String = LocalDate.now().toString(),
    val type: String = "expense",
    val currency: String = "IDR",
    val isLoading: Boolean = false,
    val success: Boolean = false,
    val error: String? = null,
)

val EXPENSE_CATEGORIES = listOf("Makanan", "Transport", "Belanja", "Tagihan", "Hiburan", "Kesehatan", "Pendidikan", "Lainnya")
val INCOME_CATEGORIES = listOf("Gaji", "Bonus", "Freelance", "Investasi", "Bisnis", "Lainnya")

@HiltViewModel
class CreateTransactionViewModel @Inject constructor(
    private val txRepo: TransactionRepository,
    private val session: SessionManager,
) : ViewModel() {

    private val _state = MutableStateFlow(CreateTransactionState())
    val state = _state.asStateFlow()

    fun appendDigit(digit: Int) {
        _state.update { s ->
            val newCents = s.cents * 10 + digit
            if (newCents > 999_999_999_999L) s else s.copy(cents = newCents)
        }
    }

    fun backspace() = _state.update { it.copy(cents = it.cents / 10) }
    fun setCategory(v: String) = _state.update { it.copy(category = v) }
    fun setDescription(v: String) = _state.update { it.copy(description = v) }
    fun setDate(v: String) = _state.update { it.copy(date = v) }
    fun setType(v: String) = _state.update { it.copy(type = v, category = "") }

    fun create() {
        val s = _state.value
        if (s.cents == 0L || s.category.isBlank()) {
            _state.update { it.copy(error = "Masukkan jumlah dan kategori") }
            return
        }
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            val token = session.token.first() ?: return@launch
            val workspaceId = session.workspaceId.first().takeIf { !it.isNullOrBlank() } ?: return@launch
            val desc = s.description.takeIf { it.isNotBlank() }
            txRepo.createTransaction(token, workspaceId, s.cents.toDouble(), s.currency, s.category, desc, s.date, s.type)
                .onSuccess { _state.update { it.copy(success = true, isLoading = false) } }
                .onFailure { err -> _state.update { it.copy(error = err.message, isLoading = false) } }
        }
    }
}
