package dev.msdqn.finance.di

import dagger.Binds
import dagger.Module
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import dev.msdqn.finance.data.repository.AuthRepositoryImpl
import dev.msdqn.finance.data.repository.InsightRepositoryImpl
import dev.msdqn.finance.data.repository.TransactionRepositoryImpl
import dev.msdqn.finance.data.repository.WorkspaceRepositoryImpl
import dev.msdqn.finance.domain.repo.AuthRepository
import dev.msdqn.finance.domain.repo.InsightRepository
import dev.msdqn.finance.domain.repo.TransactionRepository
import dev.msdqn.finance.domain.repo.WorkspaceRepository
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
abstract class RepositoryModule {

    @Binds @Singleton
    abstract fun bindAuthRepository(impl: AuthRepositoryImpl): AuthRepository

    @Binds @Singleton
    abstract fun bindWorkspaceRepository(impl: WorkspaceRepositoryImpl): WorkspaceRepository

    @Binds @Singleton
    abstract fun bindTransactionRepository(impl: TransactionRepositoryImpl): TransactionRepository

    @Binds @Singleton
    abstract fun bindInsightRepository(impl: InsightRepositoryImpl): InsightRepository
}
