package dev.msdqn.finance.presentation.nav

import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import dev.msdqn.finance.presentation.screens.auth.LoginScreen
import dev.msdqn.finance.presentation.screens.auth.RegisterScreen
import dev.msdqn.finance.presentation.screens.home.HomeScreen
import dev.msdqn.finance.presentation.screens.insights.InsightsScreen
import dev.msdqn.finance.presentation.screens.settings.SettingsScreen
import dev.msdqn.finance.presentation.screens.splash.SplashScreen
import dev.msdqn.finance.presentation.screens.statements.StatementsScreen
import dev.msdqn.finance.presentation.screens.transaction.CreateTransactionScreen
import dev.msdqn.finance.presentation.screens.transaction.TransactionScreen
import dev.msdqn.finance.presentation.screens.workspace.CreateWorkspaceScreen
import dev.msdqn.finance.presentation.screens.workspace.WorkspaceDetailScreen
import dev.msdqn.finance.presentation.screens.workspace.WorkspaceScreen
import dev.msdqn.finance.presentation.ui.BottomNavBar

private val bottomNavRoutes = setOf(
    Routes.HOME,
    Routes.TRANSACTIONS,
    Routes.STATEMENTS,
    Routes.INSIGHTS,
    Routes.SETTINGS,
)

@Composable
fun AppNavHost(navController: NavHostController = rememberNavController()) {
    val backStack by navController.currentBackStackEntryAsState()
    val currentRoute = backStack?.destination?.route
    val showBottomNav = currentRoute in bottomNavRoutes

    Scaffold { innerPadding ->
        Box(modifier = Modifier.fillMaxSize()) {
            NavHost(
                navController = navController,
                startDestination = Routes.SPLASH,
                modifier = Modifier.padding(innerPadding),
                enterTransition = { slideInHorizontally { it } + fadeIn() },
                exitTransition = { slideOutHorizontally { -it } + fadeOut() },
                popEnterTransition = { slideInHorizontally { -it } + fadeIn() },
                popExitTransition = { slideOutHorizontally { it } + fadeOut() },
            ) {
                composable(Routes.SPLASH) {
                    SplashScreen(
                        onNavigateToHome = { navController.navigate(Routes.HOME) { popUpTo(Routes.SPLASH) { inclusive = true } } },
                        onNavigateToLogin = { navController.navigate(Routes.LOGIN) { popUpTo(Routes.SPLASH) { inclusive = true } } },
                    )
                }
                composable(Routes.LOGIN) {
                    LoginScreen(
                        onNavigateToHome = { navController.navigate(Routes.HOME) { popUpTo(Routes.LOGIN) { inclusive = true } } },
                        onNavigateToRegister = { navController.navigate(Routes.REGISTER) },
                    )
                }
                composable(Routes.REGISTER) {
                    RegisterScreen(
                        onNavigateToHome = { navController.navigate(Routes.HOME) { popUpTo(Routes.REGISTER) { inclusive = true } } },
                        onNavigateToLogin = { navController.popBackStack() },
                    )
                }
                composable(Routes.HOME) {
                    HomeScreen(navController = navController)
                }
                composable(Routes.TRANSACTIONS) {
                    TransactionScreen(
                        onNavigateToCreate = { navController.navigate(Routes.CREATE_TRANSACTION) },
                        onBack = { navController.popBackStack() },
                    )
                }
                composable(Routes.CREATE_TRANSACTION) {
                    CreateTransactionScreen(onBack = { navController.popBackStack() })
                }
                composable(Routes.STATEMENTS) {
                    StatementsScreen()
                }
                composable(Routes.WORKSPACES) {
                    WorkspaceScreen(
                        onNavigateToCreate = { navController.navigate(Routes.CREATE_WORKSPACE) },
                        onNavigateToDetail = { id -> navController.navigate(Routes.workspaceDetail(id)) },
                    )
                }
                composable(Routes.CREATE_WORKSPACE) {
                    CreateWorkspaceScreen(onBack = { navController.popBackStack() })
                }
                composable(
                    route = Routes.WORKSPACE_DETAIL,
                    arguments = listOf(navArgument("workspaceId") { type = NavType.StringType }),
                ) { backStackEntry ->
                    val workspaceId = backStackEntry.arguments?.getString("workspaceId") ?: ""
                    WorkspaceDetailScreen(workspaceId = workspaceId, onBack = { navController.popBackStack() })
                }
                composable(Routes.INSIGHTS) {
                    InsightsScreen()
                }
                composable(Routes.SETTINGS) {
                    SettingsScreen(
                        onNavigateToLogin = { navController.navigate(Routes.LOGIN) { popUpTo(0) { inclusive = true } } },
                        onNavigateToWorkspaces = { navController.navigate(Routes.WORKSPACES) },
                    )
                }
            }

            if (showBottomNav) {
                BottomNavBar(
                    currentRoute = currentRoute,
                    onNavigate = { route ->
                        navController.navigate(route) {
                            popUpTo(Routes.HOME) { saveState = true }
                            launchSingleTop = true
                            restoreState = true
                        }
                    },
                    modifier = Modifier.align(Alignment.BottomCenter),
                )
            }
        }
    }
}
