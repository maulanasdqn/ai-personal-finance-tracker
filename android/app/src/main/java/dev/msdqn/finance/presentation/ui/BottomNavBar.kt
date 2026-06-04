package dev.msdqn.finance.presentation.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AutoGraph
import androidx.compose.material.icons.filled.Description
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Receipt
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import dev.msdqn.finance.presentation.nav.Routes
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.NavBlack

private data class NavItem(val route: String, val icon: ImageVector, val label: String)

private val navItems = listOf(
    NavItem(Routes.HOME, Icons.Filled.Home, "Beranda"),
    NavItem(Routes.TRANSACTIONS, Icons.Filled.Receipt, "Transaksi"),
    NavItem(Routes.STATEMENTS, Icons.Filled.Description, "Rekening"),
    NavItem(Routes.INSIGHTS, Icons.Filled.AutoGraph, "Insights"),
    NavItem(Routes.SETTINGS, Icons.Filled.Settings, "Pengaturan"),
)

@Composable
fun BottomNavBar(
    currentRoute: String?,
    onNavigate: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    Box(
        modifier = modifier
            .navigationBarsPadding()
            .padding(horizontal = 24.dp, vertical = 16.dp)
            .clip(RoundedCornerShape(40.dp))
            .background(NavBlack)
            .padding(horizontal = 12.dp, vertical = 10.dp),
    ) {
        Row(
            horizontalArrangement = Arrangement.spacedBy(4.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            navItems.forEach { item ->
                val selected = currentRoute == item.route
                Box(
                    modifier = Modifier
                        .clip(CircleShape)
                        .background(if (selected) CardWhite.copy(alpha = 0.15f) else Color.Transparent)
                        .clickable { onNavigate(item.route) }
                        .padding(12.dp),
                    contentAlignment = Alignment.Center,
                ) {
                    Icon(
                        imageVector = item.icon,
                        contentDescription = item.label,
                        modifier = Modifier.size(22.dp),
                        tint = if (selected) CardWhite else CardWhite.copy(alpha = 0.5f),
                    )
                }
            }
        }
    }
}
