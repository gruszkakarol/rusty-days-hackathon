mod conway;
mod draw;
mod soundgen;

use std::{thread, time};

use anyhow::Result;

use conway::Grid;
use draw::Renderer;
use soundgen::SoundGenerator;

fn main() -> Result<()> {
    let mut grid = Grid::random();
    let renderer = Renderer::new();
    let soundgen = SoundGenerator::new();

    loop {
        renderer.draw(&grid)?;

        thread::sleep(time::Duration::from_millis(100));

        for organism in grid.organisms() {
            if organism.is_dying() {
                soundgen.play(organism.cell_count() as u32)?;
            }
        }

        grid.next_gen();
    }
}
