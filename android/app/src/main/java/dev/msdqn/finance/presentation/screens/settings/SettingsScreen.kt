package dev.msdqn.finance.presentation.screens.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.presentation.theme.MintBackground
import dev.msdqn.finance.presentation.theme.TextSecondary

@Composable
fun SettingsScreen(onNavigateToLogin: () -> Unit, viewModel: SettingsViewModel = hiltViewModel()) {
    val email by viewModel.email.collectAsState()
    val fullName by viewModel.fullName.collectAsState()

    Column(
        modifier = Modifier.fillMaxSize().background(MintBackground).padding(24.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text("Settings", fontSize = 24.sp, fontWeight = FontWeight.Bold)
        Spacer(Modifier.height(8.dp))
        fullName?.let {
            Text("Name", style = MaterialTheme.typography.labelSmall, color = TextSecondary)
            Text(it, style = MaterialTheme.typography.bodyLarge, fontWeight = FontWeight.Medium)
        }
        email?.let {
            Text("Email", style = MaterialTheme.typography.labelSmall, color = TextSecondary)
            Text(it, style = MaterialTheme.typography.bodyLarge, fontWeight = FontWeight.Medium)
        }
        Spacer(Modifier.height(24.dp))
        Button(
            onClick = { viewModel.logout(onNavigateToLogin) },
            modifier = Modifier.fillMaxWidth().height(52.dp),
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error),
            shape = RoundedCornerShape(28.dp),
        ) {
            Text("Sign Out", fontWeight = FontWeight.SemiBold)
        }
    }
}
