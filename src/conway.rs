//! This contains the Conway's Game of Life logic. Notably:
//! * generating the next generation from the previous one,
//! * producing an iterator over all the organisms in a generation,
//! * counting the cells of an organism, and
//! * determining whether an organism is dying.

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

pub struct Grid {
    rows: [[bool; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn next_gen(&mut self) {
        todo!();
    }

    pub fn organisms() -> Organisms {
        todo!();
    }
}

pub struct Organism;

impl Organism {
    pub fn is_dying(&self) -> bool {
        todo!();
    }

    pub fn cell_count(&self) -> usize {
        todo!();
    }
}

/// Iterator over a grid's organisms.
pub struct Organisms;