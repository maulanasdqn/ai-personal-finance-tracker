package dev.msdqn.finance.domain.model

data class AuthToken(
    val token: String,
    val userId: String,
    val email: String,
    val fullName: String,
)
