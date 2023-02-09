use yew::prelude::*;

pub enum Msg {}

pub struct DashboardPage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for DashboardPage {
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
                <div class="sidebar">
                    <p class="sideApp">{"Application : "}</p>
                </div>

                <div class="header">
                    <a href="">
                        <img class="logo_header" src="images/Arbitra_Horizontal2.png"/> 
                    </a>
                    <div class="header-right">
                        <a class="" href="#home">
                            <img class="logo_right_header" src="images/bell.png"/>
                        </a>
                        <a href="">
                            <img class="logo_right_header" src="images/interrogation.png"/>
                        </a>
                        <a href="">
                            <img class="logo_right_header" src="images/user.png"/>
                        </a>
                    </div>
                </div>


                <div class="header-2">
                    <div class="header-left">
                        <a href="">
                            <img class="logo_header" src="images/menu-burger.png"/> 
                        </a> 
                        
                        <a href="">
                            {"Dashboard"}
                        </a>
                    </div>

                    <div class="header-right">
                        <form action="">
                            <input type="text" placeholder="Search..." name="search"/>
                        </form>
                        <button type="button" class="btn btn-primary">
                            <img src="images/search.png" alt="search icon"/> 
                        </button>
                    </div>


                </div>

                
                //     <div class="header-left">
                //         <div class="header-left-a">
                //             
                //         </div>
                        
                //         <div class="header-left-a">
                //             <a href="">
                //                 {"Dashboard"}
                //             </a>
                //         </div>
                //     </div>

                //     <div class="header-right">
                //         <form action="">
                //         <input type="text" placeholder="Search..." name="search"/>
                //         <button type="submit">
                //             <img src="images/search.png" alt="search icon"/> 
                //         </button>
                //         </form>

                //     </div>
                // </div>
               

            </div>
        }
    }
}