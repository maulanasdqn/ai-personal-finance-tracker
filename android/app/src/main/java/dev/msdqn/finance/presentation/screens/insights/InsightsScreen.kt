package dev.msdqn.finance.presentation.screens.insights

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.domain.model.AiInsight
import dev.msdqn.finance.presentation.theme.ButtonPastel
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.DarkSurface
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.TextSecondary
import dev.msdqn.finance.presentation.ui.ErrorBanner

@Composable
fun InsightsScreen(viewModel: InsightsViewModel = hiltViewModel()) {
    val uiState by viewModel.uiState.collectAsState()
    var dateFrom by rememberSaveable { mutableStateOf("") }
    var dateTo by rememberSaveable { mutableStateOf("") }

    LaunchedEffect(Unit) { viewModel.load() }

    LazyColumn(modifier = Modifier.fillMaxSize().background(MintBackground).padding(bottom = 100.dp)) {
        item {
            Text(
                "AI Insights",
                modifier = Modifier.fillMaxWidth().padding(horizontal = 24.dp, vertical = 20.dp),
                fontSize = 24.sp,
                fontWeight = FontWeight.Bold,
            )
        }

        item {
            ErrorBanner(
                message = uiState.error,
                modifier = Modifier.padding(horizontal = 16.dp).padding(bottom = 8.dp),
            )
        }

        item {
            Card(
                modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 6.dp),
                colors = CardDefaults.cardColors(containerColor = CardWhite),
                shape = RoundedCornerShape(16.dp),
                elevation = CardDefaults.cardElevation(0.dp),
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text(
                        "Generate Insights",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.SemiBold,
                    )
                    Spacer(Modifier.height(12.dp))
                    OutlinedTextField(
                        value = dateFrom,
                        onValueChange = { dateFrom = it },
                        label = { Text("Dari Tanggal") },
                        placeholder = { Text("2024-01-01") },
                        modifier = Modifier.fillMaxWidth(),
                        singleLine = true,
                        shape = RoundedCornerShape(12.dp),
                    )
                    Spacer(Modifier.height(8.dp))
                    OutlinedTextField(
                        value = dateTo,
                        onValueChange = { dateTo = it },
                        label = { Text("Sampai Tanggal") },
                        placeholder = { Text("2024-01-01") },
                        modifier = Modifier.fillMaxWidth(),
                        singleLine = true,
                        shape = RoundedCornerShape(12.dp),
                    )
                    Spacer(Modifier.height(12.dp))
                    Button(
                        onClick = { viewModel.generate(dateFrom, dateTo) },
                        modifier = Modifier.fillMaxWidth().height(48.dp),
                        enabled = !uiState.isLoading,
                        colors = ButtonDefaults.buttonColors(containerColor = ButtonPastel, contentColor = CardWhite),
                        shape = RoundedCornerShape(24.dp),
                    ) {
                        if (uiState.isLoading) {
                            CircularProgressIndicator(modifier = Modifier.height(20.dp), color = CardWhite, strokeWidth = 2.dp)
                        } else {
                            Text("Generate", fontWeight = FontWeight.SemiBold)
                        }
                    }
                }
            }
            Spacer(Modifier.height(8.dp))
        }

        items(uiState.insights) { insight ->
            InsightCard(insight)
        }

        if (uiState.insights.isEmpty() && !uiState.isLoading) {
            item {
                Text(
                    "Belum ada insights. Buat transaksi untuk mendapatkan analisis AI.",
                    modifier = Modifier.padding(24.dp),
                    style = MaterialTheme.typography.bodyMedium,
                    color = TextSecondary,
                )
            }
        }
    }
}

@Composable
private fun InsightCard(insight: AiInsight) {
    Card(
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 6.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        shape = RoundedCornerShape(16.dp),
        elevation = CardDefaults.cardElevation(0.dp),
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                insight.insightType.uppercase(),
                style = MaterialTheme.typography.labelSmall,
                color = DarkSurface,
                fontWeight = FontWeight.Bold,
            )
            Spacer(Modifier.height(6.dp))
            Text(insight.content, style = MaterialTheme.typography.bodyMedium)
            Spacer(Modifier.height(6.dp))
            Text("${insight.dateFrom} — ${insight.dateTo}", style = MaterialTheme.typography.labelSmall, color = TextSecondary)
        }
    }
}
