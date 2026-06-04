package dev.msdqn.finance.presentation.screens.workspace

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
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
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.domain.model.Workspace
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.NavyPrimary
import dev.msdqn.finance.presentation.theme.WarmCream

@Composable
fun WorkspaceScreen(onNavigateToCreate: () -> Unit, viewModel: WorkspaceViewModel = hiltViewModel()) {
    val uiState by viewModel.uiState.collectAsState()

    LaunchedEffect(Unit) { viewModel.load() }

    Box(modifier = Modifier.fillMaxSize().background(WarmCream)) {
        LazyColumn(modifier = Modifier.fillMaxSize().padding(bottom = 100.dp)) {
            item {
                Text("Workspaces", modifier = Modifier.fillMaxWidth().padding(horizontal = 24.dp, vertical = 16.dp), style = MaterialTheme.typography.headlineMedium)
            }
            items(uiState.workspaces) { ws ->
                WorkspaceCard(workspace = ws, onSelect = { viewModel.selectWorkspace(ws.id) })
            }
        }
        FloatingActionButton(
            onClick = onNavigateToCreate,
            modifier = Modifier.align(Alignment.BottomEnd).padding(end = 24.dp, bottom = 120.dp),
            containerColor = NavyPrimary,
            shape = RoundedCornerShape(16.dp),
        ) {
            Icon(Icons.Filled.Add, contentDescription = "Create workspace", tint = androidx.compose.ui.graphics.Color.White)
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun WorkspaceCard(workspace: Workspace, onSelect: () -> Unit) {
    Card(
        onClick = onSelect,
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 4.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        shape = RoundedCornerShape(12.dp),
        elevation = CardDefaults.cardElevation(2.dp),
    ) {
        androidx.compose.foundation.layout.Column(modifier = Modifier.padding(16.dp)) {
            Text(workspace.name, style = MaterialTheme.typography.titleMedium)
            workspace.description?.let { Text(it, style = MaterialTheme.typography.bodyMedium) }
        }
    }
}
