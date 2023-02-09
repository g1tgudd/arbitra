use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/dashboard"]
    DashboardPage,
    #[to="/login"]
    LoginPage,
    #[to="/signup"]
    SignupPage,
    #[to="/"]
    HomePage,
}
