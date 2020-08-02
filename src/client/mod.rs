mod board;
mod controls;

use board::BoardView;
use controls::Controls;
use yew::prelude::*;

type Board = ();

struct State {
    boards: Vec<Board>,
    simulation_started: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            boards: Vec::new(),
            simulation_started: false,
        }
    }

    fn toggle_simulation(&mut self) {
        self.simulation_started = !self.simulation_started;
    }
}

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

pub enum Message {
    SpawnBoard,
    DeleteBoard(usize),
    ToggleSimulation,
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
            state: State::new(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::SpawnBoard => {
                // TODO: push new board when they are ready instead of a number
                self.state.boards.push(());
                true
            }
            Message::DeleteBoard(index) => {
                self.state.boards.remove(index);
                true
            }
            Message::ToggleSimulation => {
                self.state.toggle_simulation();
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let spawn_board = self.link.callback(|_| Message::SpawnBoard);
        let toggle_simulation = self.link.callback(|_| Message::ToggleSimulation);
        html! {
            <div class="app">
                <div class="boards">
                    {self.state.boards.iter().enumerate().map(|(i, b)| self.board_view(&b, i)).collect::<Html>()}
                    <button class="button add" onclick=spawn_board>
                        <i class="fas fa-plus"></i>
                    </button>
                </div>
                <Controls simulation_started=self.state.simulation_started on_click=toggle_simulation />
            </div>
        }
    }
}
