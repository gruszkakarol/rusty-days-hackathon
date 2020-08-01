use yew::prelude::*;

pub struct Board;

impl Component for Board {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="board">
                <button class="button board__close">
                    <i class="fas fa-times" />
                </button>
                <div class="board__cells">
                    {"."}
                </div>
            </div>
        }
    }
}
