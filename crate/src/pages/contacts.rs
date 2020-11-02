use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Contacts;

impl Component for Contacts {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Contacts {}
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
                    <h2>{"Contacts"}</h2>
                    <p>{"Email: sowetocon@gmail.com"}<br />
                    {"Insta: https://www.instagram.com/sowetocon/"}<br />
                    {"Facebook: https://www.facebook.com/soweto.con"}<br />
                    {"Twitter: https://twitter.com/sowetocon"}<br />
                    </p>

                </Item>
            </Container>
        }
    }
}
