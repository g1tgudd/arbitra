use yew::prelude::*;

pub enum Msg {}

pub struct HomePage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for HomePage {
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
                <h1 style="background-color: #f2f5f7; padding-left: 12%;">{ "arbitra" }</h1>
                
                <div class="leftbox">
                    <div class="redborderbox">
                        <div class="yellowborderbox">
                            <p class="arbitraPlatform">{ "Arbitra's Platform" }</p>
                            <p class="arbitraPlatformDesc">{ "Create reliable search solutions" }<br/>{ "for your needs" }</p>
                        </div>

                        <div style="display: block; border: 1px solid #9eadba; margin-bottom: 10px; margin-left: 20px; margin-right: 20px"></div>

                        <div style="margin-left: 20px; padding-bottom: 20%">
                            <p style="color: #293845; font-size: 140%; font-weight: bold;">{ "Additional Resources" }</p>
                            <p style="color: #293845; font-size: 110%;">{ "(image) Free trial and downloads" }</p>
                            <p style="color: #293845; font-size: 110%;">{ "(image) Pricing" }</p>
                        </div>
                    </div>
                </div>
                
                <div class="middlebox">
                    <p class="industryHeader">{ "Industry Use Cases" }</p>
                    <ul>
                        <li class="industryDesc">{ "Lorem Ipsum" }</li>
                        <li class="industryDesc">{ "Lorem Ipsum" }</li>
                        <li class="industryDesc">{ "Lorem Ipsum" }</li>
                        <li class="industryDesc">{ "Lorem Ipsum" }</li>
                        <li class="industryDesc">{ "Lorem Ipsum" }</li>
                    </ul>
                </div>
                
                <div class="rightbox">
                    <h2 style="text-align: left">{ "Integrations" }</h2>
                </div>
            </div>
        }
    }
}