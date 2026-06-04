package dev.msdqn.finance.domain.repo

import dev.msdqn.finance.domain.model.AuthToken

interface AuthRepository {
    suspend fun login(email: String, password: String): Result<AuthToken>
    suspend fun register(email: String, password: String, fullName: String): Result<AuthToken>
}
