use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"About"}</h2>
                    <p>{"Soweto Con is an annual conference aiming to bring technical IT talks and training on contemporary topics to the Soweto community, exposing upcoming hackers and rocket scientists to a broad kaleidoscope (no idea what this word means but Mpho Matlala assured me it is not offensive) of career paths. The conference is also a way to give back to our community through education."}</p>
                    <p>{"Topics will include Artificial Intelligence, Cyber Security, Digital Forensics, Threat Intelligence, Computer Vision, Machine Learning, Deep Learning, Blockchain, Cryptocurrencies, Cloud Computing, Neural Network, Data Science, Kubernetes, Docker and many more"}</p>
                    <p>{"Depending on availability of relevant speakers or trainers, talks and training on popular programming languages such as Rust, WASM, Python and Golang will also be offered."}</p>
                    <p>{"Soweto Con organizers desire to create an environment where everyone is welcome to participate irrespective of colour, status, gender, language, country of origin, political affiliation, choice of alcohol or attire. As the motto goes “diversity is our strength.” 
                        Although all written material will be in English, speakers and trainers will be at liberty to present in vernacular."}</p>
                        <p>{"All our participants are driven by the burning desire to continuously learn and teach."}</p>
                        <p>{"Contrary to common practices in conferences, Soweto Con shall not exalt any topic or field as superior to others. Rather, we acknowledge that everyone is a superstar in their own field."}</p>
                        <p>{"Come and be part of Soweto Con, the first conference to guarantee you to learn something new, something you never thought existed in your lifetime."}</p>

                </Item>
            </Container>
        }
    }
}
