package dev.msdqn.finance.presentation.screens.transaction

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.Backspace
import androidx.compose.material3.FilterChip
import androidx.compose.material3.FilterChipDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.presentation.theme.ButtonPastel
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.DarkSurface
import dev.msdqn.finance.presentation.theme.ExpenseRed
import dev.msdqn.finance.presentation.theme.IncomeGreen
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.MintSurface
import dev.msdqn.finance.presentation.theme.TextSecondary
import dev.msdqn.finance.presentation.ui.ErrorBanner
import dev.msdqn.finance.presentation.util.formatRupiahLong

@Composable
fun CreateTransactionScreen(onBack: () -> Unit, viewModel: CreateTransactionViewModel = hiltViewModel()) {
    val state by viewModel.state.collectAsState()
    LaunchedEffect(state.success) { if (state.success) onBack() }

    val categories = if (state.type == "income") INCOME_CATEGORIES else EXPENSE_CATEGORIES

    Column(modifier = Modifier.fillMaxSize().background(MintBackground).statusBarsPadding()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp, vertical = 4.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = null, tint = DarkSurface)
            }
            Text("Transaksi Baru", style = MaterialTheme.typography.titleLarge, modifier = Modifier.weight(1f), textAlign = TextAlign.Center)
            Spacer(Modifier.size(48.dp))
        }

        Spacer(Modifier.height(8.dp))

        Row(modifier = Modifier.fillMaxWidth().padding(horizontal = 20.dp), horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            listOf("expense" to "Pengeluaran", "income" to "Pemasukan").forEach { (t, label) ->
                FilterChip(
                    selected = state.type == t,
                    onClick = { viewModel.setType(t) },
                    label = { Text(label) },
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = if (t == "income") IncomeGreen else ExpenseRed,
                        selectedLabelColor = CardWhite,
                        containerColor = MintSurface,
                        labelColor = DarkSurface,
                    ),
                    border = null,
                )
            }
        }

        Spacer(Modifier.height(12.dp))

        Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 24.dp), contentAlignment = Alignment.Center) {
            Text(
                text = formatRupiahLong(state.cents),
                fontSize = 40.sp,
                fontWeight = FontWeight.Bold,
                color = if (state.type == "income") IncomeGreen else ExpenseRed,
                textAlign = TextAlign.Center,
            )
        }

        Spacer(Modifier.height(12.dp))

        Text("Kategori", style = MaterialTheme.typography.labelSmall, color = TextSecondary, modifier = Modifier.padding(horizontal = 20.dp))
        Spacer(Modifier.height(6.dp))
        Row(
            modifier = Modifier.fillMaxWidth().horizontalScroll(rememberScrollState()).padding(horizontal = 20.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            categories.forEach { cat ->
                FilterChip(
                    selected = state.category == cat,
                    onClick = { viewModel.setCategory(cat) },
                    label = { Text(cat, fontSize = 13.sp) },
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = ButtonPastel,
                        selectedLabelColor = CardWhite,
                        containerColor = MintSurface,
                        labelColor = DarkSurface,
                    ),
                    border = null,
                )
            }
        }

        Spacer(Modifier.height(12.dp))

        Column(modifier = Modifier.padding(horizontal = 20.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
            OutlinedTextField(
                value = state.description,
                onValueChange = { viewModel.setDescription(it) },
                label = { Text("Deskripsi (opsional)") },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true,
                shape = RoundedCornerShape(14.dp),
            )
            OutlinedTextField(
                value = state.date,
                onValueChange = { viewModel.setDate(it) },
                label = { Text("Tanggal (YYYY-MM-DD)") },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true,
                shape = RoundedCornerShape(14.dp),
            )
            ErrorBanner(message = state.error)
        }

        Spacer(Modifier.weight(1f))

        val keys = listOf(listOf(1, 2, 3), listOf(4, 5, 6), listOf(7, 8, 9), listOf(-2, 0, -1))
        Column(modifier = Modifier.padding(horizontal = 20.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            keys.forEach { row ->
                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    row.forEach { key ->
                        Box(
                            modifier = Modifier.weight(1f).height(52.dp).clip(RoundedCornerShape(14.dp))
                                .background(if (key == -2) Color.Transparent else MintSurface)
                                .clickable(enabled = key != -2) { if (key == -1) viewModel.backspace() else viewModel.appendDigit(key) },
                            contentAlignment = Alignment.Center,
                        ) {
                            when (key) {
                                -1 -> Icon(Icons.AutoMirrored.Filled.Backspace, contentDescription = "Hapus", tint = TextSecondary)
                                -2 -> {}
                                else -> Text(key.toString(), fontSize = 20.sp, fontWeight = FontWeight.SemiBold, color = DarkSurface)
                            }
                        }
                    }
                }
            }
        }

        Spacer(Modifier.height(12.dp))

        Box(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 20.dp).clip(RoundedCornerShape(28.dp))
                .background(ButtonPastel).clickable(enabled = !state.isLoading) { viewModel.create() }.padding(vertical = 16.dp),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                text = if (state.isLoading) "Menyimpan..." else "Buat Transaksi",
                color = CardWhite,
                fontWeight = FontWeight.SemiBold,
                fontSize = 16.sp,
            )
        }
        Spacer(Modifier.height(20.dp))
    }
}
