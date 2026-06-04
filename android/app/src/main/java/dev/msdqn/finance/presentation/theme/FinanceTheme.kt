package dev.msdqn.finance.presentation.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable

private val LightColorScheme = lightColorScheme(
    primary = NavyPrimary,
    onPrimary = CardWhite,
    primaryContainer = Lavender100,
    onPrimaryContainer = Navy900,
    secondary = PinkAccent,
    onSecondary = CardWhite,
    secondaryContainer = PinkLight,
    onSecondaryContainer = Navy900,
    tertiary = NavyLight,
    onTertiary = CardWhite,
    background = WarmCream,
    onBackground = TextPrimary,
    surface = CardWhite,
    onSurface = TextPrimary,
    surfaceVariant = SurfaceVariant,
    onSurfaceVariant = TextSecondary,
    outline = TextSecondary,
    error = ExpenseRed,
    onError = CardWhite,
)

@Composable
fun FinanceTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = LightColorScheme,
        typography = Typography,
        content = content,
    )
}
