use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/index"]
    IndexPage,
    #[to="/dashboard"]
    DashboardPage,
    #[to="/login"]
    LoginPage,
    #[to="/signup"]
    SignupPage,
    #[to="/"]
    HomePage,
}
