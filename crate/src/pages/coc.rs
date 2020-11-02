use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct CoC;

impl Component for CoC {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CoC {}
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
                    <h2>{"Code of Conduct"}</h2>
                    <p>{"Be nice to other people. If someone is not nice to you, remind them to be nice. If all fails, report them to us immediately. 
                        If you don’t know what being nice is, read along (courtesy of 0xcon.co.za):"}</p>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"When you come to the conference, we ask that you:"}
                    <ul><li>{"Participate in an authentic and active way. In doing so, you contribute to the health and longevity of our community."}</li></ul>
                    <ul><li>{"Exercise consideration and respect in your speech and actions."}</li></ul>
                    <ul><li>{"Refrain from demeaning, discriminatory, or harassing behaviour and speech."}</li></ul>
                    <ul><li>{"Be mindful of your surroundings and of your fellow participants"}</li></ul>
                    </p>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"On the other hand, generally offensive behaviour is unwelcome and unacceptable. Particularly:"}
                    <ul><li>{"Violence, threats of violence or violent language directed against another person."}</li></ul>
                    <ul><li>{"Sexist,racist,homophobic,trans-phobic,ableist or otherwise discriminatory jokes and language"}</li></ul>
                    <ul><li>{"Personal insults, particularly those related to gender,sexual orientation, race, religion, or disability."}</li></ul>
                    <ul><li>{"Inappropriate photography or recording."}</li></ul>
                    <ul><li>{"Inappropriate physical contact. You should have someone’s consent before touching them."}</li></ul>
                    <ul><li>{"Unwelcome sexual attention. This includes, sexualised comments or jokes; inappropriate touching, groping, and unwelcome sexual advances"}</li></ul>
                    <ul><li>{"Deliberate intimidation, stalking or following (online or in person)"}</li></ul>
                    <ul><li>{"Advocating for, or encouraging, any of the above behaviour"}</li></ul>
                    <ul><li>{"Sustained disruption of community events, including talks and presentations."}</li></ul>
                    </p>
                </Item>
                </Item>
                    
                </Item>
            </Container>
        }
    }
}
