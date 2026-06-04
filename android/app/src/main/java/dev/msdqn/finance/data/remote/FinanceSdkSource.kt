package dev.msdqn.finance.data.remote

import dev.msdqn.finance.FinanceSdk
import org.json.JSONObject
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class FinanceSdkSource @Inject constructor(private val sdk: FinanceSdk) {

    private fun parse(json: String): JSONObject = JSONObject(json)

    fun login(baseUrl: String, email: String, password: String): Result<JSONObject> = runCatching {
        parse(sdk.login(baseUrl, email, password))
    }

    fun register(baseUrl: String, email: String, password: String, fullName: String): Result<JSONObject> = runCatching {
        parse(sdk.register(baseUrl, email, password, fullName))
    }

    fun listWorkspaces(baseUrl: String, token: String): Result<JSONObject> = runCatching {
        parse(sdk.listWorkspaces(baseUrl, token))
    }

    fun createWorkspace(baseUrl: String, token: String, name: String, description: String?): Result<JSONObject> = runCatching {
        parse(sdk.createWorkspace(baseUrl, token, name, description ?: ""))
    }

    fun getWorkspace(baseUrl: String, token: String, id: String): Result<JSONObject> = runCatching {
        parse(sdk.getWorkspace(baseUrl, token, id))
    }

    fun updateWorkspace(baseUrl: String, token: String, id: String, name: String?, description: String?): Result<JSONObject> = runCatching {
        parse(sdk.updateWorkspace(baseUrl, token, id, name ?: "", description ?: ""))
    }

    fun deleteWorkspace(baseUrl: String, token: String, id: String): Result<JSONObject> = runCatching {
        parse(sdk.deleteWorkspace(baseUrl, token, id))
    }

    fun listMembers(baseUrl: String, token: String, workspaceId: String): Result<JSONObject> = runCatching {
        parse(sdk.listMembers(baseUrl, token, workspaceId))
    }

    fun addMember(baseUrl: String, token: String, workspaceId: String, email: String): Result<JSONObject> = runCatching {
        parse(sdk.addMember(baseUrl, token, workspaceId, email))
    }

    fun listTransactions(
        baseUrl: String, token: String, workspaceId: String,
        category: String?, type: String?, from: String?, to: String?,
        limit: Int?, offset: Int?,
    ): Result<JSONObject> = runCatching {
        parse(sdk.listTransactions(baseUrl, token, workspaceId, category ?: "", type ?: "", from ?: "", to ?: "", limit?.toString() ?: "", offset?.toString() ?: ""))
    }

    fun createTransaction(
        baseUrl: String, token: String, workspaceId: String,
        amount: Double, currency: String, category: String,
        description: String?, date: String, type: String,
    ): Result<JSONObject> = runCatching {
        parse(sdk.createTransaction(baseUrl, token, workspaceId, amount.toString(), currency, category, description ?: "", date, type))
    }

    fun deleteTransaction(baseUrl: String, token: String, workspaceId: String, id: String): Result<JSONObject> = runCatching {
        parse(sdk.deleteTransaction(baseUrl, token, workspaceId, id))
    }

    fun listInsights(baseUrl: String, token: String, workspaceId: String, type: String?): Result<JSONObject> = runCatching {
        parse(sdk.listInsights(baseUrl, token, workspaceId, type ?: ""))
    }

    fun generateInsights(baseUrl: String, token: String, workspaceId: String, dateFrom: String, dateTo: String): Result<JSONObject> = runCatching {
        parse(sdk.generateInsights(baseUrl, token, workspaceId, dateFrom, dateTo))
    }
}
