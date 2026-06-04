package dev.msdqn.finance.presentation.screens.home

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.NavController
import dev.msdqn.finance.domain.model.Transaction
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.IncomeGreen
import dev.msdqn.finance.presentation.theme.Navy900
import dev.msdqn.finance.presentation.theme.NavyLight
import dev.msdqn.finance.presentation.theme.NavyPrimary
import dev.msdqn.finance.presentation.theme.WarmCream

@Composable
fun HomeScreen(navController: NavController, viewModel: HomeViewModel = hiltViewModel()) {
    val uiState by viewModel.uiState.collectAsState()
    val fullName by viewModel.fullName.collectAsState()

    LaunchedEffect(Unit) { viewModel.load() }

    LazyColumn(
        modifier = Modifier.fillMaxSize().background(WarmCream).padding(bottom = 100.dp),
    ) {
        item {
            Box(
                modifier = Modifier.fillMaxWidth().clip(RoundedCornerShape(bottomStart = 32.dp, bottomEnd = 32.dp))
                    .background(Brush.verticalGradient(listOf(Navy900, NavyPrimary, NavyLight)))
                    .padding(24.dp),
            ) {
                Column {
                    Text("Hello, ${fullName ?: "there"}", color = CardWhite, style = MaterialTheme.typography.titleLarge)
                    Spacer(Modifier.height(4.dp))
                    Text("Your financial overview", color = CardWhite.copy(alpha = 0.7f), style = MaterialTheme.typography.bodyMedium)
                    Spacer(Modifier.height(24.dp))
                    Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                        SummaryCard("Income", uiState.totalIncome, IncomeGreen)
                        SummaryCard("Expense", uiState.totalExpense, MaterialTheme.colorScheme.error)
                    }
                }
            }
        }
        item {
            Text("Recent Transactions", modifier = Modifier.padding(horizontal = 24.dp, vertical = 16.dp), style = MaterialTheme.typography.titleMedium)
        }
        items(uiState.recentTransactions) { tx ->
            TransactionItem(tx)
        }
    }
}

@Composable
private fun SummaryCard(label: String, amount: Double, color: androidx.compose.ui.graphics.Color) {
    Card(
        modifier = Modifier.padding(4.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite.copy(alpha = 0.15f)),
        shape = RoundedCornerShape(16.dp),
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(label, color = CardWhite.copy(alpha = 0.8f), style = MaterialTheme.typography.labelSmall)
            Text("$${String.format("%.2f", amount)}", color = color, style = MaterialTheme.typography.titleMedium)
        }
    }
}

@Composable
private fun TransactionItem(tx: Transaction) {
    Card(
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 4.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        shape = RoundedCornerShape(12.dp),
        elevation = CardDefaults.cardElevation(2.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column {
                Text(tx.category, style = MaterialTheme.typography.titleMedium)
                Text(tx.transactionDate, style = MaterialTheme.typography.bodyMedium)
            }
            val color = if (tx.transactionType == "income") IncomeGreen else MaterialTheme.colorScheme.error
            val sign = if (tx.transactionType == "income") "+" else "-"
            Text("$sign$${String.format("%.2f", tx.amount)}", color = color, style = MaterialTheme.typography.titleMedium)
        }
    }
}
