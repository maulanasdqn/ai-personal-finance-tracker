package dev.msdqn.finance

object FinanceSdk {
    private const val BASE_URL = "https://finance.msdqn.dev"

    init {
        System.loadLibrary("finance_android")
    }

    // Auth
    external fun register(baseUrl: String, email: String, password: String, fullName: String): String
    external fun login(baseUrl: String, email: String, password: String): String

    // Workspaces
    external fun listWorkspaces(baseUrl: String, token: String): String
    external fun createWorkspace(baseUrl: String, token: String, name: String, description: String): String
    external fun getWorkspace(baseUrl: String, token: String, workspaceId: String): String
    external fun updateWorkspace(baseUrl: String, token: String, workspaceId: String, name: String, description: String): String
    external fun deleteWorkspace(baseUrl: String, token: String, workspaceId: String): String
    external fun listMembers(baseUrl: String, token: String, workspaceId: String): String
    external fun addMember(baseUrl: String, token: String, workspaceId: String, email: String): String

    // Transactions
    external fun listTransactions(baseUrl: String, token: String, workspaceId: String, category: String, type: String, from: String, to: String, limit: String, offset: String): String
    external fun createTransaction(baseUrl: String, token: String, workspaceId: String, amount: String, currency: String, category: String, description: String, date: String, type: String): String
    external fun deleteTransaction(baseUrl: String, token: String, workspaceId: String, id: String): String

    // Bank Statements
    external fun listStatements(baseUrl: String, token: String, workspaceId: String): String
    external fun getStatement(baseUrl: String, token: String, workspaceId: String, id: String): String
    external fun uploadStatement(baseUrl: String, token: String, workspaceId: String, fileName: String, contentType: String, data: ByteArray): String

    // AI Insights
    external fun listInsights(baseUrl: String, token: String, workspaceId: String, insightType: String): String
    external fun generateInsights(baseUrl: String, token: String, workspaceId: String, dateFrom: String, dateTo: String): String

    // Convenience wrappers using default BASE_URL
    fun login(email: String, password: String) = login(BASE_URL, email, password)
    fun register(email: String, password: String, fullName: String) = register(BASE_URL, email, password, fullName)
    fun listWorkspaces(token: String) = listWorkspaces(BASE_URL, token)
    fun createWorkspace(token: String, name: String, description: String = "") = createWorkspace(BASE_URL, token, name, description)
    fun getWorkspace(token: String, workspaceId: String) = getWorkspace(BASE_URL, token, workspaceId)
    fun listTransactions(token: String, workspaceId: String, category: String = "", type: String = "", from: String = "", to: String = "", limit: Int = 50, offset: Int = 0) =
        listTransactions(BASE_URL, token, workspaceId, category, type, from, to, limit.toString(), offset.toString())
    fun createTransaction(token: String, workspaceId: String, amount: Double, currency: String = "IDR", category: String, description: String = "", date: String, type: String) =
        createTransaction(BASE_URL, token, workspaceId, amount.toString(), currency, category, description, date, type)
    fun uploadStatement(token: String, workspaceId: String, fileName: String, contentType: String, data: ByteArray) =
        uploadStatement(BASE_URL, token, workspaceId, fileName, contentType, data)
    fun generateInsights(token: String, workspaceId: String, dateFrom: String, dateTo: String) =
        generateInsights(BASE_URL, token, workspaceId, dateFrom, dateTo)
}
