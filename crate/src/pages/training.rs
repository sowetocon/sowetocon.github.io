use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Training;

impl Component for Training {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Training {}
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
                    <h2>{"Training"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                <p>{"There will be free training the entire week before the conference. Let us know if you want to give training on anything related to the conference topics. This will be free, so be prepared to spend a day teaching."}</p>
                  
                </Item>
            </Container>
        }
    }
}
