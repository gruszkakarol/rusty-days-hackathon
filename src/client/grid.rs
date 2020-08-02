use crate::conway::{Grid, CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::soundgen::SoundGenerator;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew::MouseEvent;
pub const STROKE_WIDTH: f64 = 2.0;

#[derive(Properties, Clone)]
pub struct GridProps {
    pub on_delete: Callback<MouseEvent>,
    pub grid: Grid,
}
pub struct GridView {
    link: ComponentLink<Self>,
    props: GridProps,
    canvas_ref: NodeRef,
    ctx: Option<web_sys::CanvasRenderingContext2d>,
}

impl GridView {
    fn draw_board(&self) {
        let ctx = self.ctx.as_ref().unwrap();
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#989e9e"));
        ctx.set_line_width(2.0);
        // +1 because we need that extra lines to close up the grid
        for x in 0..GRID_WIDTH + 1 {
            ctx.move_to((x * CELL_SIZE) as f64, 0.0);
            ctx.line_to((x * CELL_SIZE) as f64, (GRID_HEIGHT * CELL_SIZE) as f64);
        }

        for y in 0..GRID_HEIGHT + 1 {
            ctx.move_to(0.0, (y * CELL_SIZE) as f64);
            ctx.line_to((GRID_HEIGHT * CELL_SIZE) as f64, (y * CELL_SIZE) as f64);
        }

        ctx.stroke();
    }

    fn draw_cells(&self) {
        let ctx = self.ctx.as_ref().unwrap();
        ctx.begin_path();
        for (i, c) in self.props.grid.iter().enumerate() {
            let y = ((i / GRID_WIDTH) as f32).ceil() as usize;
            let x = i - (y * GRID_WIDTH);
            ctx.set_fill_style(&JsValue::from_str(c.color()));
            ctx.fill_rect(
                (x * CELL_SIZE) as f64,
                (y * CELL_SIZE) as f64,
                CELL_SIZE as f64,
                CELL_SIZE as f64,
            );
        }
    }

    fn draw(&self) {
        self.draw_cells();
        self.draw_board();
    }

    pub fn toggle_cell(&mut self, event: &MouseEvent) {
        let x = (event.offset_x() as f32 / CELL_SIZE as f32).floor();
        let y = (event.offset_y() as f32 / CELL_SIZE as f32).floor();
        let index = x as usize + (y as usize * GRID_WIDTH);
        log::info!("x:{} y:{} i:{}", x, y, index);
        // TODO: handle this
        let alive = self
            .props
            .grid
            .get_cell(index)
            .map(|cell| cell.alive)
            .expect("This shouldn't happen");

        self.props.grid.set_cell(index, !alive);
        self.draw();
    }

    pub fn simulate(&mut self) {
        let soundgen = SoundGenerator::new();
        self.props.grid.next_gen();
        self.draw();

        for (x, y) in self.props.grid.get_pitch_and_volume_per_subgrid() {
            soundgen.play(*x as u32).expect("Fix it");
        }
    }
}

pub enum Message {
    ClickCanvas(MouseEvent),
    Simulate,
}

impl Component for GridView {
    type Properties = GridProps;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            canvas_ref: NodeRef::default(),
            ctx: None,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::ClickCanvas(e) => self.toggle_cell(&e),
            Message::Simulate => {
                self.simulate();
            }
        }
        false
    }

    fn view(&self) -> Html {
        let delete_grid = &self.props.on_delete;
        let click_canvas = self.link.callback(|e| Message::ClickCanvas(e));
        let toggle_simulation = self.link.callback(|_| Message::Simulate);
        html! {
            <div class="grid">
                <div class="grid__controls">
                    <button class="button grid__close" onclick=delete_grid>
                        <i class="fas fa-times" />
                    </button>
                    <button class="button grid__play" onclick=toggle_simulation>
                        {if self.props.grid.stopped {
                            html! { <i class="fas fa-stop"></i> }
                        } else {
                            html!{ <i class="fas fa-play"></i> }
                        }}
                    </button>
                </div>
                <canvas onclick=click_canvas id="canvas" ref=self.canvas_ref.clone() class="grid__cells" />
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                canvas.set_height((GRID_HEIGHT * CELL_SIZE) as u32);
                canvas.set_width((GRID_WIDTH * CELL_SIZE) as u32);
                self.ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .ok();
                self.draw();
            }
        }
    }
}
