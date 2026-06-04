package dev.msdqn.finance.domain.repo

import dev.msdqn.finance.domain.model.BankStatement

interface BankStatementRepository {
    suspend fun listStatements(token: String, workspaceId: String): Result<List<BankStatement>>
    suspend fun getStatement(token: String, workspaceId: String, id: String): Result<BankStatement>
}
