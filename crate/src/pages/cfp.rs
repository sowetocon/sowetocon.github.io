use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct CfP;

impl Component for CfP {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CfP {}
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
                    <h2>{"Call for Papers (CFP)"}</h2>
                    <p>{"Submission is open to anyone. Your research can be anything cool that you want to share with the community even if it is not listed in the Topics section. Email sowetocon@gmail.com. Deadline for submission is a month before the actual conference date."}</p>
                </Item>
            </Container>
        }
    }
}
