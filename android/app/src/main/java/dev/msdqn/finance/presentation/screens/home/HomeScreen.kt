package dev.msdqn.finance.presentation.screens.home

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.AutoGraph
import androidx.compose.material.icons.filled.NotificationsNone
import androidx.compose.material.icons.filled.Receipt
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.NavController
import dev.msdqn.finance.domain.model.Transaction
import dev.msdqn.finance.presentation.nav.Routes
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.DarkSurface
import dev.msdqn.finance.presentation.theme.ExpenseRed
import dev.msdqn.finance.presentation.theme.IncomeGreen
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.MintSurface
import dev.msdqn.finance.presentation.theme.TextMuted
import dev.msdqn.finance.presentation.theme.TextSecondary
import dev.msdqn.finance.presentation.theme.YellowCard
import java.text.NumberFormat
import java.util.Locale

private val fmt: NumberFormat = NumberFormat.getNumberInstance(Locale.US).apply {
    minimumFractionDigits = 2
    maximumFractionDigits = 2
}

@Composable
fun HomeScreen(navController: NavController, viewModel: HomeViewModel = hiltViewModel()) {
    val uiState by viewModel.uiState.collectAsState()
    val fullName by viewModel.fullName.collectAsState()

    LaunchedEffect(Unit) { viewModel.load() }

    LazyColumn(
        modifier = Modifier.fillMaxSize().background(MintBackground),
        contentPadding = PaddingValues(bottom = 120.dp),
    ) {
        item {
            Column(modifier = Modifier.padding(horizontal = 20.dp)) {
                Spacer(Modifier.statusBarsPadding())
                Spacer(Modifier.height(12.dp))
                // Header
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                        Box(
                            modifier = Modifier.size(40.dp).clip(CircleShape).background(DarkSurface),
                            contentAlignment = Alignment.Center,
                        ) {
                            Text(
                                text = fullName?.firstOrNull()?.uppercaseChar()?.toString() ?: "?",
                                color = CardWhite,
                                fontWeight = FontWeight.Bold,
                                fontSize = 16.sp,
                            )
                        }
                        Column {
                            Text("Hi, ${fullName?.substringBefore(" ") ?: "there"}!", style = MaterialTheme.typography.titleLarge)
                        }
                    }
                    Row {
                        IconButton(onClick = {}) {
                            Icon(Icons.Filled.NotificationsNone, contentDescription = null, tint = DarkSurface)
                        }
                        IconButton(onClick = { navController.navigate(Routes.INSIGHTS) }) {
                            Icon(Icons.Filled.AutoGraph, contentDescription = null, tint = DarkSurface)
                        }
                    }
                }

                Spacer(Modifier.height(20.dp))

                // Balance Card
                Box(
                    modifier = Modifier.fillMaxWidth().clip(RoundedCornerShape(24.dp)).background(YellowCard).padding(24.dp),
                ) {
                    Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.fillMaxWidth()) {
                        Text(uiState.currency, fontWeight = FontWeight.SemiBold, fontSize = 14.sp, color = DarkSurface.copy(alpha = 0.6f))
                        Spacer(Modifier.height(4.dp))
                        Text(
                            text = "$${fmt.format(uiState.totalBalance)}",
                            fontSize = 38.sp,
                            fontWeight = FontWeight.Bold,
                            color = DarkSurface,
                            textAlign = TextAlign.Center,
                        )
                        Spacer(Modifier.height(4.dp))
                        val changeSign = if (uiState.todayChange >= 0) "+" else ""
                        Text(
                            text = "$changeSign$${fmt.format(uiState.todayChange)} today",
                            fontSize = 13.sp,
                            fontWeight = FontWeight.Medium,
                            color = if (uiState.todayChange >= 0) IncomeGreen else ExpenseRed,
                        )
                    }
                }

                Spacer(Modifier.height(20.dp))

                // Quick actions
                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceEvenly) {
                    QuickAction(Icons.Filled.Add, "Add", onClick = { navController.navigate(Routes.CREATE_TRANSACTION) })
                    QuickAction(Icons.Filled.Receipt, "Transactions", onClick = { navController.navigate(Routes.TRANSACTIONS) })
                    QuickAction(Icons.Filled.AutoGraph, "Insights", onClick = { navController.navigate(Routes.INSIGHTS) })
                }

                Spacer(Modifier.height(24.dp))

                // Latest Transactions header
                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween, verticalAlignment = Alignment.CenterVertically) {
                    Text("Latest Transactions", style = MaterialTheme.typography.titleMedium)
                    Text(
                        "See All",
                        style = MaterialTheme.typography.labelLarge,
                        color = TextSecondary,
                        modifier = Modifier.clickable { navController.navigate(Routes.TRANSACTIONS) },
                    )
                }
                Spacer(Modifier.height(12.dp))
            }
        }

        items(uiState.recentTransactions) { tx ->
            TransactionRow(tx = tx, modifier = Modifier.padding(horizontal = 20.dp))
            Spacer(Modifier.height(8.dp))
        }
    }
}

@Composable
private fun QuickAction(icon: ImageVector, label: String, onClick: () -> Unit) {
    Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.clickable(onClick = onClick)) {
        Box(
            modifier = Modifier.size(52.dp).clip(CircleShape).background(MintSurface),
            contentAlignment = Alignment.Center,
        ) {
            Icon(icon, contentDescription = label, tint = DarkSurface, modifier = Modifier.size(22.dp))
        }
        Spacer(Modifier.height(6.dp))
        Text(label, style = MaterialTheme.typography.labelSmall, color = TextSecondary)
    }
}

@Composable
fun TransactionRow(tx: Transaction, modifier: Modifier = Modifier) {
    val isIncome = tx.transactionType == "income"
    val amountColor = if (isIncome) IncomeGreen else ExpenseRed
    val amountSign = if (isIncome) "+" else "-"
    val initial = tx.category.firstOrNull()?.uppercaseChar()?.toString() ?: "?"
    val bgColor = if (isIncome) IncomeGreen.copy(alpha = 0.15f) else ExpenseRed.copy(alpha = 0.12f)
    val iconColor = if (isIncome) IncomeGreen else ExpenseRed

    Card(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(16.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        elevation = CardDefaults.cardElevation(0.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(14.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Box(
                modifier = Modifier.size(40.dp).clip(CircleShape).background(bgColor),
                contentAlignment = Alignment.Center,
            ) {
                Text(initial, color = iconColor, fontWeight = FontWeight.Bold, fontSize = 15.sp)
            }
            Column(modifier = Modifier.weight(1f)) {
                Text(tx.category, style = MaterialTheme.typography.titleMedium)
                Text(tx.transactionType.replaceFirstChar { it.uppercase() }, style = MaterialTheme.typography.bodyMedium)
            }
            Text(
                text = "$amountSign${tx.currency} ${fmt.format(tx.amount)}",
                fontWeight = FontWeight.SemiBold,
                fontSize = 15.sp,
                color = amountColor,
            )
        }
    }
}
