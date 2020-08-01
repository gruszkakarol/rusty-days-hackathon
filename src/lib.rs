#![recursion_limit = "512"]
mod client;
mod conway;
mod renderer;
mod soundgen;

use client::App;
use conway::Grid;
use renderer::Renderer;
use soundgen::SoundGenerator;
use std::{thread, time};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();

    let mut grid = Grid::random();
    let renderer = Renderer::new();
    let soundgen = SoundGenerator::new();

    loop {
        renderer.draw(&grid).expect("Fix it");

        thread::sleep(time::Duration::from_millis(100));

        for organism in grid.organisms() {
            if organism.is_dying() {
                soundgen.play(organism.cell_count() as u32).expect("Fix it");
            }
        }

        grid.next_gen();
    }

    Ok(())
}
