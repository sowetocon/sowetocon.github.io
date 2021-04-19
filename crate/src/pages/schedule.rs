use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Schedule;

impl Component for Schedule {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Schedule {}
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
                    <h2>{"Schedule"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                //<dl>{"The conference starts on the 23rd until the 24th April 2021."}</dl>
                <dt><b>{"Friday: Day 1"}</b></dt>
                <p>{"[+] 11:00: Keynote ->  On love, health and Ubuntu"}</p>
                <p>{"[+] 11:30: Entrepreneurship ->  The cyber hustlers startup guide "}</p>
                <p>{"[+] 12:30: Break"}</p>
                <p>{"[+] 13:30: Artificial Intelligence -> Guide to building your frst model"}</p>
                <p>{"[+] 14:20: Blockchain -> Understanding Smart contracts, Defi & NFTs"}</p>
                <p>{"[+] 15:30: Trading -> From zero to quant"}</p>
                   
                <dt><b>{"Saturday: Day 2"}</b></dt>
                <p>{"[+] 11:00: SecIntel -> Hackers and Spies - A comparison"}</p>
                <p>{"[+] 11:40: SecDevOps -> Agile Threat Modelling"}</p>
                <p>{"[+] 12:30: Break"}</p>
                <p>{"[+] 13:30: Programming -> A look at Rust security"}</p>
                <p>{"[+] 14:20: Hacking -> Whitebox fuzzing Rust and C/CPP APIs"}</p>
                <p>{"[+] 15:30: Hacking -> Whitebox web hacking PHP and Java Apps"}</p>
                </Item>
                <dl>{"We might have a Soweto Tour, follow @sowetocon for updates."}</dl>

            </Container>
        }
    }
}
