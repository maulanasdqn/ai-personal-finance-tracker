package dev.msdqn.finance.presentation.screens.workspace

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
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
import dev.msdqn.finance.domain.model.Workspace
import dev.msdqn.finance.presentation.theme.ButtonPastel
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.MintBackground

@Composable
fun WorkspaceScreen(
    onNavigateToCreate: () -> Unit,
    onNavigateToDetail: (String) -> Unit = {},
    viewModel: WorkspaceViewModel = hiltViewModel(),
) {
    val uiState by viewModel.uiState.collectAsState()

    LaunchedEffect(Unit) { viewModel.load() }

    Box(modifier = Modifier.fillMaxSize().background(MintBackground)) {
        LazyColumn(modifier = Modifier.fillMaxSize().padding(bottom = 100.dp)) {
            item {
                Text(
                    "Workspaces",
                    modifier = Modifier.fillMaxWidth().padding(horizontal = 24.dp, vertical = 20.dp),
                    fontSize = 24.sp,
                    fontWeight = FontWeight.Bold,
                )
            }
            items(uiState.workspaces) { ws ->
                WorkspaceCard(workspace = ws, onTap = { onNavigateToDetail(ws.id) })
            }
        }
        FloatingActionButton(
            onClick = onNavigateToCreate,
            modifier = Modifier.align(Alignment.BottomEnd).padding(end = 24.dp, bottom = 120.dp),
            containerColor = ButtonPastel,
            shape = RoundedCornerShape(16.dp),
        ) {
            Icon(Icons.Filled.Add, contentDescription = "Buat workspace", tint = CardWhite)
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun WorkspaceCard(workspace: Workspace, onTap: () -> Unit) {
    Card(
        onClick = onTap,
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 6.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        shape = RoundedCornerShape(16.dp),
        elevation = CardDefaults.cardElevation(0.dp),
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(workspace.name, style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.SemiBold)
            workspace.description?.let { Text(it, style = MaterialTheme.typography.bodyMedium) }
        }
    }
}
