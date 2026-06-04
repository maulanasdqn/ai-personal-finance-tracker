package dev.msdqn.finance.presentation.screens.splash

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import dev.msdqn.finance.presentation.theme.DarkSurface
import dev.msdqn.finance.presentation.theme.MintBackground

@Composable
fun SplashScreen(
    onNavigateToHome: () -> Unit,
    onNavigateToLogin: () -> Unit,
    viewModel: SplashViewModel = hiltViewModel(),
) {
    val isAuthenticated by viewModel.isAuthenticated.collectAsState()

    LaunchedEffect(isAuthenticated) {
        when (isAuthenticated) {
            true -> onNavigateToHome()
            false -> onNavigateToLogin()
            null -> Unit
        }
    }

    Box(
        modifier = Modifier.fillMaxSize().background(MintBackground),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = "Finance",
            color = DarkSurface,
            fontSize = 32.sp,
            fontWeight = FontWeight.Bold,
        )
    }
}
