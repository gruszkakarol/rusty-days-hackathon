use yew::prelude::*;
use yew::MouseEvent;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ControlsProps {
    pub on_click: Callback<MouseEvent>,
    pub simulation_started: bool,
}
pub struct Controls {
    props: ControlsProps,
    simulation_started: bool,
}

impl Component for Controls {
    type Properties = ControlsProps;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            simulation_started: props.simulation_started,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.simulation_started = props.simulation_started;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let toggle_simulation = &self.props.on_click;
        html! {
            <div class="controls">
                <button class="button state" onclick=toggle_simulation>
                {if self.simulation_started {
                    html! { <i class="fas fa-stop"></i> }
                } else {
                    html!{ <i class="fas fa-play"></i> }
                }}
                </button>
            </div>
        }
    }
}
