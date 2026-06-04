package dev.msdqn.finance.presentation.screens.transaction

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
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.FilterChipDefaults
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SwipeToDismissBox
import androidx.compose.material3.SwipeToDismissBoxValue
import androidx.compose.material3.Text
import androidx.compose.material3.rememberSwipeToDismissBoxState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.presentation.theme.ButtonPastel
import dev.msdqn.finance.presentation.theme.ButtonPastelRed
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.DarkSurface
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.MintSurface
import dev.msdqn.finance.presentation.theme.SectionLabel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TransactionScreen(
    onNavigateToCreate: () -> Unit,
    onBack: () -> Unit = {},
    viewModel: TransactionViewModel = hiltViewModel(),
) {
    val uiState by viewModel.uiState.collectAsState()
    LaunchedEffect(Unit) { viewModel.load() }

    val filters = listOf("all" to "Semua", "income" to "Pemasukan", "expense" to "Pengeluaran")

    Box(modifier = Modifier.fillMaxSize().background(MintBackground)) {
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = PaddingValues(bottom = 120.dp),
        ) {
            item {
                Column(modifier = Modifier.padding(horizontal = 20.dp)) {
                    Spacer(Modifier.statusBarsPadding())
                    Spacer(Modifier.height(12.dp))
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        IconButton(onClick = onBack) {
                            Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = null, tint = DarkSurface)
                        }
                        Text(
                            "Transaksi",
                            style = MaterialTheme.typography.titleLarge,
                            modifier = Modifier.weight(1f).padding(start = 4.dp),
                        )
                        IconButton(onClick = {}) {
                            Icon(Icons.Filled.Search, contentDescription = null, tint = DarkSurface)
                        }
                    }
                    Spacer(Modifier.height(8.dp))
                    Row(horizontalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(8.dp)) {
                        filters.forEach { (key, label) ->
                            FilterChip(
                                selected = uiState.filter == key,
                                onClick = { viewModel.setFilter(key) },
                                label = { Text(label) },
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
                    Spacer(Modifier.height(8.dp))
                }
            }

            uiState.grouped.forEach { (label, txs) ->
                item {
                    Text(
                        text = label,
                        style = MaterialTheme.typography.labelLarge,
                        color = SectionLabel,
                        modifier = Modifier.padding(horizontal = 20.dp, vertical = 8.dp),
                    )
                }
                items(txs, key = { it.id }) { tx ->
                    val dismissState = rememberSwipeToDismissBoxState(
                        confirmValueChange = { value ->
                            if (value == SwipeToDismissBoxValue.EndToStart) {
                                viewModel.delete(tx.id)
                                true
                            } else {
                                false
                            }
                        },
                    )
                    SwipeToDismissBox(
                        state = dismissState,
                        enableDismissFromStartToEnd = false,
                        backgroundContent = {
                            Box(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(horizontal = 20.dp)
                                    .background(ButtonPastelRed, RoundedCornerShape(16.dp))
                                    .padding(end = 20.dp),
                                contentAlignment = Alignment.CenterEnd,
                            ) {
                                Icon(
                                    Icons.Filled.Delete,
                                    contentDescription = "Hapus",
                                    tint = CardWhite,
                                    modifier = Modifier.size(24.dp),
                                )
                            }
                        },
                    ) {
                        TransactionCard(tx = tx, modifier = Modifier.padding(horizontal = 20.dp))
                    }
                    Spacer(Modifier.height(8.dp))
                }
            }
        }

        FloatingActionButton(
            onClick = onNavigateToCreate,
            modifier = Modifier.align(Alignment.BottomEnd).padding(end = 24.dp, bottom = 112.dp),
            containerColor = ButtonPastel,
            shape = RoundedCornerShape(16.dp),
        ) {
            Icon(Icons.Filled.Add, contentDescription = "Tambah", tint = CardWhite)
        }
    }
}
