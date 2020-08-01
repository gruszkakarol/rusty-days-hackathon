mod board;

use board::Board;
use yew::prelude::*;

// #[derive(Properties, Clone)]
// pub struct Props {
//     pub children: Children,
// }

pub struct App {
    link: ComponentLink<Self>,
    boards: Vec<i32>,
}

impl App {
    fn board_view(&self) -> Html {
        html! {
            <Board />
        }
    }
}

pub enum Message {
    SpawnBoard,
}

impl Component for App {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            boards: Vec::new(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::SpawnBoard => {
                // TODO: push new board when they are ready instead of a number
                self.boards.push(0);
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Message::SpawnBoard);
        html! {
            <div class="app">
                <div class="boards">
                    {self.boards.iter().map(|b| self.board_view()).collect::<Html>()}
                    <button class="button add" onclick=onclick>
                        <i class="fas fa-plus"></i>
                    </button>
                </div>
            </div>
        }
    }
}
