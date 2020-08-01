//! This provides the Renderer type which draws a Game of Life grid to some
//! sort of GUI or TUI.

use thiserror::Error;

use crate::conway::Grid;

type Result<V> = std::result::Result<V, RenderError>;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Renderer {
        Renderer
    }

    pub fn draw(&self, grid: &Grid) -> Result<()> {
        todo!()
    }
}

// For how to use this, see: https://docs.rs/thiserror/1.0.20/thiserror/
#[derive(Error, Debug)]
pub enum RenderError {
    
}