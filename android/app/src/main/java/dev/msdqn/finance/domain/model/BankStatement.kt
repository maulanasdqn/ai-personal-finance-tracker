package dev.msdqn.finance.domain.model

data class BankStatement(
    val id: String,
    val workspaceId: String,
    val fileName: String,
    val fileUrl: String,
    val uploadedAt: String,
)
