use yew::prelude::*;

pub enum Msg {}
pub struct SignupPage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for SignupPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

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
            <div class="container-fluid h-100"> 
                <div class="row" style="height: 100vh">
                    <div style="background-color:#A73034" class="col-md text-light">
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-center"> 
                            <img src="images/Arbitra Full2 Wht.png" class="card-img-top" style="width: 200px" alt="..."/>

                        </div>
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-md-start"> 
                            <h2><strong>{ "Get Started For Free."}</strong></h2>
                            <h4>{ "Search API for your every need." }</h4>
                        </div>
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-between flex-row">
                            <div class="p-2"> 
                                <img src="images/predictive-chart.png" class="card-img-top" style="width: 60px" alt="..."/>
                            </div>
                            
                            <div  class="p-2"> 
                                <h4>{ "Manage your data." }</h4>
                                <p class="card-text" style="text-align: justify"> { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
                                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, 
                                quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. " } </p>                              
                            </div> 
                        </div>
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-between flex-row">
                            <div  class="p-2">
                                <img src="images/database.png" class="card-img-top" style="width: 60px" alt="..."/>
                            </div>

                            <div  class="p-2"> 
                                <h4>{ "Analytics & Services" }</h4>
                                <p class="card-text" style="text-align: justify"> { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
                                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, 
                                quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. " } </p>   
                            </div>
                            
                        </div>
                        // ===========================================

                    
                    </div>
                    <div class="col-md-9 d-flex justify-content-center align-items-center">
                        <div class="card d-flex justify-content-center p-5 mx-4" style="width: 40vw"> 
                            <div class="d-flex justify-content-between align-items-center"> 
                                <h1 style="padding-bottom: 5%; font-family: 'Alexandria', sans-serif;">{ "Sign Up" }</h1>
                                <div>
                                    <p style="height: 5rem">{ "Already have an account? " } 
                                    <a href="/login" class="link-primary" style="color:#A73034">{ "Log in" }</a></p>
                                </div>
                                
                            </div>
                                // ===========================================
                            <div> 
                                <input type="text" id="emailInput" class="form-control p-3 my-2 " placeholder="Email" style="height: 3rem"/>
                                <input type="password" id="passwordInput" class="form-control p-3 my-2 " placeholder="Password" style="height: 3rem"/>
                            </div>
                               // ===========================================
                            <div class="py-3"> 
                                <input type="checkbox" class="form-check-input" id="tc"/>
                                <label class="form-check-label px-2" for="tc">{ "I accept Arbitra's terms and conditions" }</label>

                            </div>
                                // ===========================================
                            
                            <button type="button" class="btn text-light" style="background-color:#A73034">{ "Sign Up" } </button>
                            
                            
                            <div class="mt-3 mb-1"> 
                                <p class="text-center">{ "or sign up with:" }</p>
                            </div>
                            <div style="height:40px;" class="mx-auto"> 
                                <button type="button" class="btn btn-link btn-floating">
                                    <img src="images/Github.png" alt="github image" style="width: 30px"/> 
                                </button>

                                <button type="button" class="btn btn-link btn-floating">
                                    <img src="images/Google.png" alt="google image" style="width: 30px"/> 
                                </button>
                            </div>
                                
                            
                        
                        </div>

                        
                    </div>
                </div>
            </div>
        }
    }
}