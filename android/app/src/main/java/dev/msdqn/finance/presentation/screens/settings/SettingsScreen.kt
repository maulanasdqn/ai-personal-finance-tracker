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
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.presentation.theme.WarmCream

@Composable
fun SettingsScreen(onNavigateToLogin: () -> Unit, viewModel: SettingsViewModel = hiltViewModel()) {
    val email by viewModel.email.collectAsState()
    val fullName by viewModel.fullName.collectAsState()

    Column(
        modifier = Modifier.fillMaxSize().background(WarmCream).padding(24.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text("Settings", style = MaterialTheme.typography.headlineMedium)
        Spacer(Modifier.height(8.dp))
        fullName?.let { Text("Name: $it", style = MaterialTheme.typography.bodyLarge) }
        email?.let { Text("Email: $it", style = MaterialTheme.typography.bodyLarge) }
        Spacer(Modifier.height(24.dp))
        Button(
            onClick = { viewModel.logout(onNavigateToLogin) },
            modifier = Modifier.fillMaxWidth(),
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error),
            shape = RoundedCornerShape(12.dp),
        ) {
            Text("Sign Out")
        }
    }
}
