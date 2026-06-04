package dev.msdqn.finance.data.local

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class SessionManager @Inject constructor(private val dataStore: DataStore<Preferences>) {

    companion object {
        const val DEFAULT_BASE_URL = "https://finance.msdqn.dev"
    }

    private val TOKEN_KEY = stringPreferencesKey("auth_token")
    private val USER_ID_KEY = stringPreferencesKey("user_id")
    private val EMAIL_KEY = stringPreferencesKey("email")
    private val FULL_NAME_KEY = stringPreferencesKey("full_name")
    private val BASE_URL_KEY = stringPreferencesKey("base_url")
    private val WORKSPACE_ID_KEY = stringPreferencesKey("workspace_id")

    val token: Flow<String?> = dataStore.data.map { it[TOKEN_KEY] }
    val userId: Flow<String?> = dataStore.data.map { it[USER_ID_KEY] }
    val email: Flow<String?> = dataStore.data.map { it[EMAIL_KEY] }
    val fullName: Flow<String?> = dataStore.data.map { it[FULL_NAME_KEY] }
    val baseUrl: Flow<String> = dataStore.data.map {
        val stored = it[BASE_URL_KEY] ?: DEFAULT_BASE_URL
        if (stored.endsWith("/api/v1") || stored.endsWith("/api/v1/")) DEFAULT_BASE_URL else stored
    }
    val workspaceId: Flow<String?> = dataStore.data.map { it[WORKSPACE_ID_KEY] }

    suspend fun saveSession(token: String, userId: String, email: String, fullName: String) {
        dataStore.edit {
            it[TOKEN_KEY] = token
            it[USER_ID_KEY] = userId
            it[EMAIL_KEY] = email
            it[FULL_NAME_KEY] = fullName
        }
    }

    suspend fun saveBaseUrl(url: String) {
        dataStore.edit { it[BASE_URL_KEY] = url }
    }

    suspend fun saveWorkspaceId(id: String) {
        dataStore.edit { it[WORKSPACE_ID_KEY] = id }
    }

    suspend fun clearSession() {
        dataStore.edit { it.clear() }
    }
}
