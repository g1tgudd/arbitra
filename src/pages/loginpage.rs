use yew::prelude::*;
use std::rc::Rc;
use yewdux::prelude::*;

use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};

use crate::store::store:: {
    CounterStore,
    CounterOutput,
    CounterInput,
    State,
};

pub enum Msg {
    State(Rc<State>),
    Output(CounterOutput),
    Login,
}

pub struct LoginPage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    dispatch: Dispatch<CounterStore>,
    username: Option<String>,
    value: i64,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dispatch = {
            let on_state = link.callback(Msg::State);
            let on_output = link.callback(Msg::Output);

            Dispatch::bridge(on_state, on_output)
        };

        Self {
            link,
            dispatch,
            username: None,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                ConsoleService::info("page app.rs");
                ConsoleService::info(&format!("state is {:?}", state));
                self.username= state.username.clone();
                true
            }
            
            Msg::Output(msg) => {
                match msg { //HANDLE OUTPUT
                    CounterOutput::Doubled(n) => {
                        ConsoleService::info(&format!("count doubled would be {:?}", n));
                        true
                    }
                    CounterOutput::AddFive(n) => {
                        ConsoleService::info(&format!("count plus five would be {:?}", n));
                        true
                    }
                     _ => {
                        ConsoleService::info(&format!("ignored"));
                        false
                    }
                }
            }

            Msg::Login => {
                self.dispatch.send(CounterInput::UpdateUsername(String::from("bruce wayne")));
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
        html! {
            <div>
                
                //{--------------------Body--------------------}//
                <img class="arbitraLogo" src="images/Arbitra_Full2.png" alt="logo image"/>
                <div class="loginBox">
                    <h1 style="padding-bottom: 5%; font-family: 'Alexandria', sans-serif;">{ "Log in" }</h1>
                    
                    <div style="padding-bottom: 24px">
                        <input type="email" id="useremail" placeholder="Email address" />
                    </div>

                    <div style="padding-bottom: 22px">
                        <input type="password" id="userpassword" placeholder="Password" />
                    </div>

                    <div style="padding-bottom: 8%">
                        <div>
                            //Checkbox

                            <div>
                                <input class="form-check-input" type="checkbox" value="" id="form2Example31" />
                                <label class="form-check-label" for="form2Example31"> { "Remember me" } </label>
                                <a href="#!" style="float:right">{ "Forgot password?" }</a> 
                            </div>
                        </div>
                    </div>

                    // Submit button
                    <button class="submitBtn" type="button" onclick=self.link.callback(|_| Msg::Login)>{ "Login" }</button>

                    //Register button
                    <div class="text-center" style="padding-top: 20px">
                        <p>{ "Not a member? " }<a href="/signup">{ "Register" }</a></p>
                    </div>

                    <div style="text-align: center; padding-bottom: 5%">
                        <p>{ "or sign up with:" }</p>
                        <button type="button" class="btn btn-link btn-floating mx-1">
                            <img src="images/Github.png" alt="github image" style="width: 30px"/> 
                        </button>
                        <button type="button" class="btn btn-link btn-floating mx-1">
                            <img src="images/img6.jpg" alt="google image" style="width: 30px"/> 
                        </button>
                    </div>

                </div>
                //{--------------------Body--------------------}//

            </div>
        }
    }
}