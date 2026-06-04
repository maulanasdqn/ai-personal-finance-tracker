package dev.msdqn.finance.presentation.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable

private val LightColorScheme = lightColorScheme(
    primary = DarkSurface,
    onPrimary = CardWhite,
    primaryContainer = YellowCard,
    onPrimaryContainer = DarkSurface,
    secondary = IncomeGreen,
    onSecondary = CardWhite,
    background = MintBackground,
    onBackground = TextPrimary,
    surface = CardWhite,
    onSurface = TextPrimary,
    surfaceVariant = MintSurface,
    onSurfaceVariant = TextSecondary,
    error = ExpenseRed,
    onError = CardWhite,
    outline = TextMuted,
)

@Composable
fun FinanceTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = LightColorScheme,
        typography = Typography,
        content = content,
    )
}
