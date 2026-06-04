package dev.msdqn.finance.presentation.util

import java.text.NumberFormat
import java.util.Locale

private val idLocale = Locale("id", "ID")

fun formatRupiah(amount: Double): String {
    val fmt = NumberFormat.getNumberInstance(idLocale).apply {
        minimumFractionDigits = 0
        maximumFractionDigits = 0
    }
    return "Rp ${fmt.format(amount)}"
}

fun formatRupiahLong(amount: Long): String = formatRupiah(amount.toDouble())
