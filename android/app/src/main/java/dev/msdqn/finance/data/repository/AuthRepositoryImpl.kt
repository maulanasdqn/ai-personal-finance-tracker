package dev.msdqn.finance.data.repository

import dev.msdqn.finance.data.local.SessionManager
import dev.msdqn.finance.data.remote.FinanceSdkSource
import dev.msdqn.finance.domain.model.AuthToken
import dev.msdqn.finance.domain.repo.AuthRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.withContext
import javax.inject.Inject

class AuthRepositoryImpl @Inject constructor(
    private val source: FinanceSdkSource,
    private val session: SessionManager,
) : AuthRepository {

    override suspend fun login(email: String, password: String): Result<AuthToken> = withContext(Dispatchers.IO) {
        val baseUrl = session.baseUrl.first()
        source.login(baseUrl, email, password).mapCatching { json ->
            val data = json.getJSONObject("data")
            val token = AuthToken(
                token = data.getString("token"),
                userId = data.getString("user_id"),
                email = data.getString("email"),
                fullName = data.getString("full_name"),
            )
            session.saveSession(token.token, token.userId, token.email, token.fullName)
            token
        }
    }

    override suspend fun register(email: String, password: String, fullName: String): Result<AuthToken> = withContext(Dispatchers.IO) {
        val baseUrl = session.baseUrl.first()
        source.register(baseUrl, email, password, fullName).mapCatching { json ->
            val data = json.getJSONObject("data")
            val token = AuthToken(
                token = data.getString("token"),
                userId = data.getString("user_id"),
                email = data.getString("email"),
                fullName = data.getString("full_name"),
            )
            session.saveSession(token.token, token.userId, token.email, token.fullName)
            token
        }
    }
}
