package dev.msdqn.finance.presentation.nav

object Routes {
    const val SPLASH = "splash"
    const val LOGIN = "login"
    const val REGISTER = "register"
    const val HOME = "home"
    const val TRANSACTIONS = "transactions"
    const val CREATE_TRANSACTION = "create_transaction"
    const val TRANSACTION_DETAIL = "transaction_detail/{id}"
    const val WORKSPACES = "workspaces"
    const val CREATE_WORKSPACE = "create_workspace"
    const val WORKSPACE_DETAIL = "workspace_detail/{workspaceId}"
    const val INSIGHTS = "insights"
    const val STATEMENTS = "statements"
    const val SETTINGS = "settings"

    fun workspaceDetail(workspaceId: String) = "workspace_detail/$workspaceId"
}
