package dev.msdqn.finance.presentation.screens.workspace

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
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.domain.model.WorkspaceMember
import dev.msdqn.finance.presentation.theme.ButtonPastel
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.DarkSurface
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.MintSurface
import dev.msdqn.finance.presentation.theme.SectionLabel
import dev.msdqn.finance.presentation.theme.TextSecondary
import dev.msdqn.finance.presentation.ui.ErrorBanner

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun WorkspaceDetailScreen(
    workspaceId: String,
    onBack: () -> Unit,
    viewModel: WorkspaceDetailViewModel = hiltViewModel(),
) {
    val uiState by viewModel.uiState.collectAsState()

    LaunchedEffect(workspaceId) { viewModel.load(workspaceId) }

    Column(modifier = Modifier.fillMaxSize().background(MintBackground)) {
        TopAppBar(
            title = { Text(uiState.workspace?.name ?: "Detail Workspace") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = null, tint = DarkSurface)
                }
            },
            colors = TopAppBarDefaults.topAppBarColors(containerColor = MintBackground),
        )

        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = PaddingValues(bottom = 32.dp),
        ) {
            item {
                ErrorBanner(
                    message = uiState.error,
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
                )
            }

            item {
                Button(
                    onClick = {
                        viewModel.selectWorkspace(workspaceId)
                        onBack()
                    },
                    modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp).height(50.dp),
                    colors = ButtonDefaults.buttonColors(containerColor = ButtonPastel, contentColor = CardWhite),
                    shape = RoundedCornerShape(24.dp),
                ) {
                    Text("Pilih Workspace", fontWeight = FontWeight.SemiBold)
                }
                Spacer(Modifier.height(20.dp))
            }

            item {
                Text(
                    "Anggota",
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                    color = SectionLabel,
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
                )
            }

            if (uiState.isLoading) {
                item {
                    Box(modifier = Modifier.fillMaxWidth().padding(24.dp), contentAlignment = Alignment.Center) {
                        CircularProgressIndicator(color = ButtonPastel)
                    }
                }
            }

            items(uiState.members) { member ->
                MemberRow(member = member)
                Spacer(Modifier.height(8.dp))
            }

            item {
                Spacer(Modifier.height(16.dp))
                Text(
                    "Undang Anggota",
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                    color = SectionLabel,
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
                )
                Column(modifier = Modifier.padding(horizontal = 16.dp)) {
                    OutlinedTextField(
                        value = uiState.inviteEmail,
                        onValueChange = { viewModel.setInviteEmail(it) },
                        label = { Text("Email") },
                        placeholder = { Text("nama@email.com") },
                        modifier = Modifier.fillMaxWidth(),
                        singleLine = true,
                        shape = RoundedCornerShape(12.dp),
                    )
                    Spacer(Modifier.height(8.dp))
                    Button(
                        onClick = { viewModel.invite(workspaceId) },
                        modifier = Modifier.fillMaxWidth().height(50.dp),
                        enabled = !uiState.isLoading && uiState.inviteEmail.isNotBlank(),
                        colors = ButtonDefaults.buttonColors(containerColor = ButtonPastel, contentColor = CardWhite),
                        shape = RoundedCornerShape(24.dp),
                    ) {
                        Text("Undang", fontWeight = FontWeight.SemiBold)
                    }
                }
            }
        }
    }
}

@Composable
private fun MemberRow(member: WorkspaceMember) {
    Card(
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        shape = RoundedCornerShape(16.dp),
        elevation = CardDefaults.cardElevation(0.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(14.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(member.fullName, style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.Medium)
                Text(member.email, style = MaterialTheme.typography.bodySmall, color = TextSecondary)
            }
            Surface(
                color = MintSurface,
                shape = RoundedCornerShape(8.dp),
            ) {
                Text(
                    text = member.role,
                    style = MaterialTheme.typography.labelSmall,
                    color = DarkSurface,
                    fontWeight = FontWeight.Medium,
                    modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp),
                )
            }
        }
    }
}
