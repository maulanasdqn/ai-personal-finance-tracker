package dev.msdqn.finance.presentation.screens.transaction

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import dev.msdqn.finance.domain.model.Transaction
import dev.msdqn.finance.presentation.theme.CardWhite
import dev.msdqn.finance.presentation.theme.ExpenseRed
import dev.msdqn.finance.presentation.theme.IncomeGreen
import dev.msdqn.finance.presentation.theme.TextSecondary
import dev.msdqn.finance.presentation.util.formatRupiah

@Composable
fun TransactionCard(tx: Transaction, modifier: Modifier = Modifier) {
    val isIncome = tx.transactionType == "income"
    val amountColor = if (isIncome) IncomeGreen else ExpenseRed
    val amountSign = if (isIncome) "+" else "-"
    val initial = tx.category.firstOrNull()?.uppercaseChar()?.toString() ?: "?"
    val bgColor = if (isIncome) IncomeGreen.copy(alpha = 0.15f) else ExpenseRed.copy(alpha = 0.12f)
    val typeLabel = if (isIncome) "Pemasukan" else "Pengeluaran"

    Card(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(16.dp),
        colors = CardDefaults.cardColors(containerColor = CardWhite),
        elevation = CardDefaults.cardElevation(0.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(14.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Box(
                modifier = Modifier.size(42.dp).clip(CircleShape).background(bgColor),
                contentAlignment = Alignment.Center,
            ) {
                Text(initial, color = amountColor, fontWeight = FontWeight.Bold, fontSize = 16.sp)
            }
            Column(modifier = Modifier.weight(1f)) {
                Text(tx.category, style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
                Text(text = tx.description ?: typeLabel, style = MaterialTheme.typography.bodySmall, color = TextSecondary)
            }
            Text(text = "$amountSign${formatRupiah(tx.amount)}", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = amountColor)
        }
    }
}
