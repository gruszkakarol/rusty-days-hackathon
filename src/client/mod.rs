mod board;

use board::BoardView;
use yew::prelude::*;

type Board = ();

pub struct App {
    link: ComponentLink<Self>,
    boards: Vec<Board>,
}

pub enum Message {
    SpawnBoard,
    DeleteBoard(usize),
}

impl App {
    fn board_view(&self, board: &Board, index: usize) -> Html {
        let on_delete = self.link.callback(move |_| Message::DeleteBoard(index));
        html! {
            <BoardView on_delete=on_delete />
        }
    }
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
                self.boards.push(());
                true
            }
            Message::DeleteBoard(index) => {
                self.boards.remove(index);
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
                    {self.boards.iter().enumerate().map(|(i, b)| self.board_view(&b, i)).collect::<Html>()}
                    <button class="button add" onclick=onclick>
                        <i class="fas fa-plus"></i>
                    </button>
                </div>
            </div>
        }
    }
}
