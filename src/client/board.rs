use yew::prelude::*;
use yew::MouseEvent;

#[derive(Properties, Clone, PartialEq)]
pub struct BoardProps {
    pub on_delete: Callback<MouseEvent>,
}
pub struct BoardView {
    props: BoardProps,
}

impl Component for BoardView {
    type Properties = BoardProps;
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
        let on_delete = &self.props.on_delete;

        html! {
            <div class="board">
                <button class="button board__close" onclick=on_delete>
                    <i class="fas fa-times" />
                </button>
                <div class="board__cells">
                    {"."}
                </div>
            </div>
        }
    }
}
