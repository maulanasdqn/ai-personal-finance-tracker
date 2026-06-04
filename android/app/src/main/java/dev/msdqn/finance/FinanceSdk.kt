package dev.msdqn.finance

class FinanceSdk {
    external fun login(baseUrl: String, email: String, password: String): String
    external fun register(baseUrl: String, email: String, password: String, fullName: String): String
    external fun listWorkspaces(baseUrl: String, token: String): String
    external fun createWorkspace(baseUrl: String, token: String, name: String, description: String): String
    external fun getWorkspace(baseUrl: String, token: String, workspaceId: String): String
    external fun updateWorkspace(baseUrl: String, token: String, workspaceId: String, name: String, description: String): String
    external fun deleteWorkspace(baseUrl: String, token: String, workspaceId: String): String
    external fun listMembers(baseUrl: String, token: String, workspaceId: String): String
    external fun addMember(baseUrl: String, token: String, workspaceId: String, email: String): String
    external fun listTransactions(baseUrl: String, token: String, workspaceId: String, category: String, txType: String, from: String, to: String, limit: String, offset: String): String
    external fun createTransaction(baseUrl: String, token: String, workspaceId: String, amount: String, currency: String, category: String, description: String, date: String, txType: String): String
    external fun deleteTransaction(baseUrl: String, token: String, workspaceId: String, id: String): String
    external fun listStatements(baseUrl: String, token: String, workspaceId: String): String
    external fun getStatement(baseUrl: String, token: String, workspaceId: String, id: String): String
    external fun listInsights(baseUrl: String, token: String, workspaceId: String, insightType: String): String
    external fun generateInsights(baseUrl: String, token: String, workspaceId: String, dateFrom: String, dateTo: String): String

    companion object {
        init {
            System.loadLibrary("finance_sdk")
        }
    }
}
