use crate::conway::{Cell, Grid};
use yew::prelude::*;
use yew::MouseEvent;

#[derive(Properties, Clone)]
pub struct GridProps {
    pub on_delete: Callback<MouseEvent>,
    pub grid: Grid,
}
pub struct GridView {
    props: GridProps,
}

impl GridView {
    fn cell_view(&self, cell: &Cell) -> Html {
        let alive_class = if cell.alive {
            "cell__alive"
        } else {
            "cell__dead"
        };
        html! {
            <div class=vec!["cell", alive_class] />
        }
    }
}

impl Component for GridView {
    type Properties = GridProps;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let delete_grid = &self.props.on_delete;

        html! {
            <div class="grid">
                <button class="button grid__close" onclick=delete_grid>
                    <i class="fas fa-times" />
                </button>
                <div class="grid__cells">
                    // { self.props.grid.iter().map(|c| self.cell_view(c)).collect::<Html>()}
                </div>
            </div>
        }
    }
}
