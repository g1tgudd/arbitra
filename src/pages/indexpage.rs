use yew::prelude::*;

pub enum Msg {}

pub struct IndexPage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for IndexPage {
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
                //SIDEBAR SMALL START
                <div>
                    <div class="leftbox index-sidebar-small">
                        <img class="index-logo" src="images/Arbitra_LogoOnly.png"/> 
                    </div>

                    <div class="rightSideBar">
                        <p style="color: #bd3143; font-size: 2rem">{"S E A R C H"}</p>
                        <p style="margin-top: -8px">{ "Application" }</p>

                        <div class="dropdown">
                            <button class="mainmenubtn"><img class="applicationIcon" src="images/APP.png"/>{ "Scara \u{00a0} \u{23F7}"}</button>
                            <div class="dropdown-child">
                                <a href="#">{ "Link 1" }</a>
                                <a href="#">{ "Link 2" }</a>
                                <a href="#">{ "Link 3" }</a>
                            </div>
                        </div>
                        
                        <br/><br/>

                        <p class="index-directry">{ "\u{007C}\u{00a0} Index" }</p>
                        <p class="index-directry">{ "\u{007C}\u{00a0} Dictionary" }</p>
                        <p class="index-directry">{ "\u{007C}\u{00a0} Lorem Ipsum" }</p>
                        <p class="index-directry">{ "\u{007C}\u{00a0} Lorem Ipsum" }</p>
                    </div>
                </div>

                <div>
                    <div class="top-index-dashboard">

                        <div class="dropdownIndex">
                            <button class="mainmenubtnIndex">{ "INDEX NAME \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                            <div class="dropdown-childIndex">
                                <a href="#">{ "Link 1" }</a>
                                <a href="#">{ "Link 2" }</a>
                                <a href="#">{ "Link 3" }</a>
                            </div>
                        </div>

                        <div class="recordData">
                            <p class="recordNum">{ "No. of Records \u{00a0} \u{00a0} \u{00a0} \u{00a0} 1.000.000.000" }</p>
                            <p style="float: left;">{ "\u{00a0} \u{00a0} \u{00a0}" }</p>
                            <p class="recordSize">{ "Average Record Size\u{00a0} \u{00a0} \u{00a0} \u{00a0} 1.000.000.000B" }</p>
                        </div>

                        <br/><br/>

                        <div class="dropdownRecord">
                            <button class="mainmenubtnRecord">{ "New Record \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                            <div class="dropdown-childRecord">
                                <a href="#">{ "Link 1" }</a>
                                <a href="#">{ "Link 2" }</a>
                                <a href="#">{ "Link 3" }</a>
                            </div>
                        </div>

                        <div class="dropdownRecord">
                            <button class="mainmenubtnRecord">{ "Add Records \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                            <div class="dropdown-childRecord">
                                <a href="#">{ "Link 1" }</a>
                                <a href="#">{ "Link 2" }</a>
                                <a href="#">{ "Link 3" }</a>
                            </div>
                        </div>

                        <div class="dropdownRecord">
                            <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                            <div class="dropdown-childRecord">
                                <a href="#">{ "Link 1" }</a>
                                <a href="#">{ "Link 2" }</a>
                                <a href="#">{ "Link 3" }</a>
                            </div>
                        </div>

                        <img class="copyIcon" src="images/Copy Icon.png"/>
                        <img class="copyIcon" src="images/Refresh.png"/>

                    </div>

                    <div class="bottom-index-dashboard">
                        <div class="flex-container">
                            <button class="subtab-p">{ "Browse" }</button>
                            <button class="subtab-p">{ "Configuration" }</button>
                            <button class="subtab-p">{ "Replicas" }</button>
                            <button class="subtab-p">{ "Search API Records" }</button>
                            <button class="subtab-p">{ "Stats" }</button>
                            <button class="subtab-p">{ "UI Demos" }</button>
                        </div>

                        <div class="card">
                            <div class="search-bar">
                                <input class="search" type="text" placeholder="Search..." />
                            </div>
                        </div>

                    </div>
                </div>

            </div>
            //BODY END
        }
    }
}