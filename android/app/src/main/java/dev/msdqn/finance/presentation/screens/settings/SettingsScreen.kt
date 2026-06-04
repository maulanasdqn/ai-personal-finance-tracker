package dev.msdqn.finance.presentation.screens.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowForwardIos
import androidx.compose.material.icons.filled.WorkspacePremium
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.presentation.theme.ButtonPastel
import dev.msdqn.finance.presentation.theme.ButtonPastelRed
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.TextSecondary

@Composable
fun SettingsScreen(
    onNavigateToLogin: () -> Unit,
    onNavigateToWorkspaces: () -> Unit = {},
    viewModel: SettingsViewModel = hiltViewModel(),
) {
    val email by viewModel.email.collectAsState()
    val fullName by viewModel.fullName.collectAsState()
    val workspaceName by viewModel.workspaceName.collectAsState()

    Column(
        modifier = Modifier.fillMaxSize().background(MintBackground).statusBarsPadding().padding(24.dp),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        Text("Pengaturan", fontSize = 24.sp, fontWeight = FontWeight.Bold)
        Spacer(Modifier.height(16.dp))

        Card(shape = RoundedCornerShape(16.dp), colors = CardDefaults.cardColors(containerColor = CardWhite), elevation = CardDefaults.cardElevation(0.dp)) {
            Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
                fullName?.let {
                    Column {
                        Text("Nama", style = MaterialTheme.typography.labelSmall, color = TextSecondary)
                        Text(it, style = MaterialTheme.typography.bodyLarge, fontWeight = FontWeight.Medium)
                    }
                }
                email?.let {
                    Column {
                        Text("Email", style = MaterialTheme.typography.labelSmall, color = TextSecondary)
                        Text(it, style = MaterialTheme.typography.bodyLarge, fontWeight = FontWeight.Medium)
                    }
                }
            }
        }

        Spacer(Modifier.height(8.dp))

        Card(
            shape = RoundedCornerShape(16.dp),
            colors = CardDefaults.cardColors(containerColor = CardWhite),
            elevation = CardDefaults.cardElevation(0.dp),
            modifier = Modifier.clickable { onNavigateToWorkspaces() },
        ) {
            Row(
                modifier = Modifier.fillMaxWidth().padding(16.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                Icon(Icons.Filled.WorkspacePremium, contentDescription = null, tint = ButtonPastel, modifier = Modifier.size(20.dp))
                Column(modifier = Modifier.weight(1f)) {
                    Text("Workspace", style = MaterialTheme.typography.labelSmall, color = TextSecondary)
                    Text(
                        text = workspaceName ?: "Tidak ada workspace dipilih",
                        style = MaterialTheme.typography.bodyLarge,
                        fontWeight = FontWeight.Medium,
                    )
                }
                Icon(Icons.AutoMirrored.Filled.ArrowForwardIos, contentDescription = null, tint = TextSecondary, modifier = Modifier.size(14.dp))
            }
        }

        Spacer(Modifier.height(16.dp))

        Button(
            onClick = { viewModel.logout(onNavigateToLogin) },
            modifier = Modifier.fillMaxWidth().height(52.dp),
            colors = ButtonDefaults.buttonColors(containerColor = ButtonPastelRed, contentColor = CardWhite),
            shape = RoundedCornerShape(28.dp),
        ) {
            Text("Keluar", fontWeight = FontWeight.SemiBold)
        }
    }
}
