use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{
    prelude::*,
    service::RouteService,
};
use crate::router::route::AppRoute;

use crate::pages::{
    dashboardpage::DashboardPage,
    homepage::HomePage,
    loginpage::LoginPage,
    signuppage::SignupPage,
    indexpage::IndexPage
};


pub enum Msg {
    AddOne,
}

pub struct RenderGuest {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for RenderGuest {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| {
            let mut route_service = RouteService::new();
            match switch {
                // AppRoute::HomePage => {
                //     html! {
                //         <HomePage/>
                //     }
                // }
                AppRoute::LoginPage => {
                    html! {
                        <LoginPage/>
                    }
                }
                _ => {
                    route_service.set_route("/loginpage", ());
                    html!{
                        <LoginPage/>
                    }
                }
                // AppRoute::SignupPage => {
                //     html! {
                //         <SignupPage/>
                //     }
                // }
                // AppRoute::DashboardPage => {
                //     html! {
                //         <DashboardPage/>
                //     }
                // }
                // AppRoute::IndexPage => {
                //     html! {
                //         <IndexPage/>
                //     }
                // }
            }
        });
        html! {
            <div>
                <Router<AppRoute, ()> render=render/>
            </div>
        }
    }
}
