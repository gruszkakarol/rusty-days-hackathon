mod board;

use board::Board;
use yew::prelude::*;

// #[derive(Properties, Clone)]
// pub struct Props {
//     pub children: Children,
// }

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                <div class="boards">
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                    <Board />
                </div>
            </div>
        }
    }
}
