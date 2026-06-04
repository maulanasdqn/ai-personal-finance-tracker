package dev.msdqn.finance.domain.model

data class Workspace(
    val id: String,
    val name: String,
    val description: String?,
    val ownerId: String,
    val createdAt: String,
)

data class WorkspaceMember(
    val id: String,
    val workspaceId: String,
    val userId: String,
    val email: String,
    val fullName: String,
    val role: String,
    val joinedAt: String,
)
