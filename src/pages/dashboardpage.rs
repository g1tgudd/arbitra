use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};

use crate::types::var::{
    DashboardData,
};

pub enum Msg {
    RequestData,
    GetDashboardData(DashboardData),
    ResponseError(String),
}

pub struct DashboardPage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    value: i64,
    dashboarddata: DashboardData,
}

impl Component for DashboardPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            link,
            value: 0,
            dashboarddata: DashboardData {
                request_amount: None,
                ping: None,
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {
                //FETCHING...

                let request = Request::get("http://localhost:3000/dashboard_data")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<DashboardData, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetDashboardData(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                true
            }

            Msg::GetDashboardData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.dashboarddata = data;
                true
            }

            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                true
            }


        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
			self.link.send_message(Msg::RequestData);
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
                

                //HEADER TOP
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
                //HEADER TOP END
                //HEADER TOP FIXED
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
                        <div class="form-outline">
                            <input type="search" id="form1" placeholder="Search" aria-label="Search" class="form-control bg-light" />
                        </div>
                        <button type="button" class="btn btn-primary">
                            <img src="images/search.png" alt="search icon"/> 
                        </button>
                    </div>
                </div>
                //HEADER TOP FIXED END

                //SIDEBAR
                <div class="sidebar">
                    <p class="sideApp">{"Application : "}</p>
                </div> 
                //SIDEBAR END


                //DASHBOARD AREA START
                <div class= "dash-container">
                    //DASHBOARD INFO START
                    <div class="dash-info">
                        <h1> {"Welcome!"} </h1>
                        <p> {"Check what's happening with your Arbitra search Implementation."} </p>
                    </div>
                    //DASHBOARD INFO END

                    //DASHBOARD BUTTONS START
                    <div class="dash-info-buttons">
                        <button class="button button-primary">{"Export Data"}</button>
                        <button class="button button-secondary">{"API Keys"}</button>
                        <button class="button button-secondary">{"Billing"}</button>
                    </div>
                    //DASHBOARD BUTTONS END

                    //DASHBOARD CARDS START
                    <div class="dash-card">
                        <h1> {"SEARCH"} </h1>

                        //SUB CARD #1
                        <div class="sub-card">
                            <h6> {"Search Requests"} </h6>
                            <h3> {self.dashboarddata.request_amount.clone().unwrap_or_default()} </h3>
                        </div>
                        //SUB CARD END

                         //SUB CARD #2
                        <div class="sub-card">
                            <h6> {"Average Search Speed"} </h6>
                            
                            <h3> {self.dashboarddata.ping.clone().unwrap_or_default()}{"ms"} </h3>
                            
                        </div>
                        //SUB CARD END

                    </div>
                    //DASHBOARD CARDS END
                </div>
                //DASHBOARD AREA END



            </div>
            //BODY END
        }
    }
}

                // <div class="header-right">
                    //     <form action="">
                    //         <input type="text" placeholder="Search..." name="search"/>
                    //     </form>
                    //     <button type="button" class="btn btn-primary">
                    //         <img src="images/search.png" alt="search icon"/> 
                    //     </button>
                    // </div>





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
