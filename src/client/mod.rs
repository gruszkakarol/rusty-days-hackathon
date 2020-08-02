mod grid;

use crate::conway::{Conway, Grid};
use grid::GridView;
use yew::prelude::*;

struct State {
    simulation: Conway,
}

impl State {
    pub fn new() -> Self {
        State {
            simulation: Conway::new(),
        }
    }
}

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

pub enum Message {
    SpawnGrid,
    DeleteGrid(usize),
    ToggleSimulation,
}

impl App {
    fn grid_view(&self, grid: &Grid, index: usize) -> Html {
        let on_delete = self.link.callback(move |_| Message::DeleteGrid(index));
        html! {
            <GridView on_delete=on_delete grid=grid />
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
            Message::SpawnGrid => {
                // TODO: push new board when they are ready instead of a number
                self.state.simulation.add_game(Grid::random());
                true
            }
            Message::DeleteGrid(index) => {
                self.state.simulation.remove_game(index);
                true
            }
            Message::ToggleSimulation => {
                self.state.simulation.toggle();
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let spawn_grid = self.link.callback(|_| Message::SpawnGrid);
        html! {
            <div class="app">
                <div class="grids">
                    {self.state.simulation.iter().enumerate().map(|(i, g)| self.grid_view(&g, i)).collect::<Html>()}
                    <button class="button add" onclick=spawn_grid>
                        <i class="fas fa-plus"></i>
                    </button>
                </div>
            </div>
        }
    }
}
