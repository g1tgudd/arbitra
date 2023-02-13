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
                        <p style="font-family: Alexandria; color: #bd3143; font-size: 2rem">{"S E A R C H"}</p>
                        <p style="font-family: Alexandria; margin-top: -8px">{ "Application" }</p>

                        <div class="dropdown">
                            <button class="mainmenubtn"><img class="applicationIcon" src="images/APP.png"/>{ "Scara \u{00a0} \u{23F7}"}</button>
                            <div class="dropdown-child">
                                <a href="#">{ "Link 1" }</a>
                                <a href="#">{ "Link 2" }</a>
                                <a href="#">{ "Link 3" }</a>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
            //BODY END
        }
    }
}