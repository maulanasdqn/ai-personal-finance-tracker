package dev.msdqn.finance.presentation.screens.insights

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
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
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.domain.model.AiInsight
import dev.msdqn.finance.presentation.theme.Lavender100
import dev.msdqn.finance.presentation.theme.WarmCream

@Composable
fun InsightsScreen(viewModel: InsightsViewModel = hiltViewModel()) {
    val uiState by viewModel.uiState.collectAsState()

    LaunchedEffect(Unit) { viewModel.load() }

    LazyColumn(modifier = Modifier.fillMaxSize().background(WarmCream).padding(bottom = 100.dp)) {
        item {
            Text("AI Insights", modifier = Modifier.fillMaxWidth().padding(horizontal = 24.dp, vertical = 16.dp), style = MaterialTheme.typography.headlineMedium)
        }
        items(uiState.insights) { insight ->
            InsightCard(insight)
        }
        if (uiState.insights.isEmpty() && !uiState.isLoading) {
            item { Text("No insights yet. Create transactions to get AI-powered analysis.", modifier = Modifier.padding(24.dp), style = MaterialTheme.typography.bodyMedium) }
        }
    }
}

@Composable
private fun InsightCard(insight: AiInsight) {
    Card(
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 4.dp),
        colors = CardDefaults.cardColors(containerColor = Lavender100),
        shape = RoundedCornerShape(12.dp),
        elevation = CardDefaults.cardElevation(2.dp),
    ) {
        androidx.compose.foundation.layout.Column(modifier = Modifier.padding(16.dp)) {
            Text(insight.insightType.uppercase(), style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.primary)
            androidx.compose.foundation.layout.Spacer(Modifier.padding(4.dp))
            Text(insight.content, style = MaterialTheme.typography.bodyMedium)
            androidx.compose.foundation.layout.Spacer(Modifier.padding(4.dp))
            Text("${insight.dateFrom} — ${insight.dateTo}", style = MaterialTheme.typography.labelSmall)
        }
    }
}
