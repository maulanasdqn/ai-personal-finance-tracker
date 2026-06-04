package dev.msdqn.finance.presentation.screens.statements

import androidx.compose.foundation.background
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
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Description
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.domain.model.BankStatement
import dev.msdqn.finance.presentation.theme.ButtonPastel
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.MintSurface
import dev.msdqn.finance.presentation.theme.TextSecondary
import dev.msdqn.finance.presentation.ui.ErrorBanner

@Composable
fun StatementsScreen(viewModel: StatementsViewModel = hiltViewModel()) {
    val uiState by viewModel.uiState.collectAsState()

    LaunchedEffect(Unit) { viewModel.load() }

    LazyColumn(
        modifier = Modifier.fillMaxSize().background(MintBackground),
        contentPadding = PaddingValues(bottom = 120.dp),
    ) {
        item {
            Spacer(Modifier.statusBarsPadding())
            Text(
                "Rekening Koran",
                modifier = Modifier.padding(horizontal = 24.dp, vertical = 20.dp),
                fontSize = 24.sp,
                fontWeight = FontWeight.Bold,
            )
            ErrorBanner(message = uiState.error, modifier = Modifier.padding(horizontal = 16.dp).padding(bottom = 8.dp))
        }

        if (uiState.isLoading) {
            item {
                Box(modifier = Modifier.fillMaxWidth().padding(48.dp), contentAlignment = Alignment.Center) {
                    CircularProgressIndicator(color = ButtonPastel)
                }
            }
        }

        items(uiState.statements) { stmt ->
            StatementCard(stmt)
            Spacer(Modifier.height(8.dp))
        }

        if (uiState.statements.isEmpty() && !uiState.isLoading && uiState.error == null) {
            item {
                Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 24.dp, vertical = 48.dp), contentAlignment = Alignment.Center) {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Icon(Icons.Filled.Description, contentDescription = null, tint = TextSecondary, modifier = Modifier.size(48.dp))
                        Spacer(Modifier.height(12.dp))
                        Text("Belum ada rekening koran", color = TextSecondary, style = MaterialTheme.typography.bodyMedium)
                    }
                }
            }
        }
    }
}

@Composable
private fun StatementCard(stmt: BankStatement) {
    Card(
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp),
        shape = RoundedCornerShape(16.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        elevation = CardDefaults.cardElevation(0.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Box(
                modifier = Modifier.size(44.dp).background(MintSurface, RoundedCornerShape(12.dp)),
                contentAlignment = Alignment.Center,
            ) {
                Icon(Icons.Filled.Description, contentDescription = null, tint = ButtonPastel, modifier = Modifier.size(24.dp))
            }
            Column(modifier = Modifier.weight(1f).padding(start = 12.dp)) {
                Text(stmt.fileName, style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
                Text(stmt.uploadedAt.take(10), style = MaterialTheme.typography.bodySmall, color = TextSecondary)
            }
        }
    }
}
