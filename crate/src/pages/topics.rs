use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Topics;

impl Component for Topics {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Topics {}
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
                    <h2>{"The following are some of the topics that we would like to cover:"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Artificial Intelligence:"}
                    <ul><li>{"Machine Learning (ML)"}</li>
                    <li>{"Deep Learning (DL)"}</li>
                    <li>{"Natural Language Processing (NLP)"}</li>
                    <li>{"Transfer Learning (TL)"}</li>
                    <li>{"Reinforcement Learning (RL)"}</li></ul>
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Cloud Computing / DevOps / DevSecOps:"}
                    <ul><li>{"AWS / Google Cloud / Microsoft Azure"}</li>
                    <li>{"Docker"}</li>
                    <li>{"Kubernetes"}</li>
                    <li>{"CI/CD Pipelines"}</li>
                    <li>{"Microservices"}</li>
                    <li>{"AWS Lambda"}</li></ul>
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Programming Languages:"}
                    <ul><li>{"Rust"}</li>
                    <li>{"WASM"}</li>
                    <li>{"Golang"}</li>
                    <li>{"Kotlin"}</li>
                    <li>{"Flutter"}</li>
                    <li>{"Python"}</li></ul>
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Offensive security:"}
                    <ul><li>{"ICS/IoT hacking"}</li>
                    <li>{"Mobile phone hacking"}</li>
                    <li>{"Fuzzing"}</li>
                    <li>{"Red teaming techniques"}</li>
                    <li>{"Penetration testing"}</li>
                    <li>{"Code auditing"}</li>
                    <li>{"Dynamic testing"}</li>
                    <li>{"Social engineering"}</li>
                    <li>{"Reverse engineering"}</li>
                    <li>{"Bug hunting"}</li></ul>
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Quantitative Finance:"}
                    <ul><li>{"Algorithmic trading"}</li>
                    <li>{"Financial machine learning"}</li>
                    <li>{"High frequency trading"}</li></ul>
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Defensive Security:"}
                    <ul><li>{"Detection and response"}</li>
                    <li>{"Threat intelligence"}</li>
                    <li>{"Malware Analysis"}</li>
                    <li>{"Blue teaming techniques"}</li>
                    <li>{"Vulnerability disclosure handling"}</li></ul>
                    </p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Miscellaneous:"}
                    <ul><li>{"Mathematics"}</li>
                    <li>{"Physics"}</li>
                    <li>{"Cryptography"}</li>
                    <li>{"Electric or energy saving cars"}</li>
                    <li>{"Space transportation"}</li>
                    <li>{"Health, transport and education focused technologies"}</li>
                    <li>{"Anything that you deem cool, send it."}</li></ul>
                    </p>
                </Item>
            </Container>
        }
    }
}
