use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
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
<img srcset="sowetocon_bg320.png,
             sowetocon_bg480.png 1.5x,
                          sowetocon_bg640.png 2x"
                                   src="sowetocon_bg640.png"
                                        alt="Soweto Conference" />

                //<img src="./sowetocon_poster.png"/> 
                    <p><b>{"Education is the most powerful weapon which you can use to change the world. -Mandela."}</b></p>
                </Item>
            </Container>
        }
    }
}
