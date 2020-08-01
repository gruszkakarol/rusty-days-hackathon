use rand::prelude::{thread_rng, Rng};

use super::index::Index;
use super::organism::Organisms;
use super::GameError;
use super::Result;

/// width of a single grid
pub const GRID_WIDTH: usize = 500;
/// height of a single grid
pub const GRID_HEIGHT: usize = 500;

#[derive(Clone)]
pub struct Grid {
    cells: [bool; GRID_WIDTH * GRID_HEIGHT],
}

impl Grid {
    pub fn random() -> Grid {
        let mut rng = thread_rng();
        let mut cells: [bool; GRID_WIDTH * GRID_HEIGHT] = [true; GRID_WIDTH * GRID_HEIGHT];

        cells
            .iter_mut()
            .for_each(|cell| *cell = rng.gen_bool(1.2 / 3.0));

        Self { cells }
    }

    pub fn next_gen(&mut self) {
        let mut new_generation: [bool; GRID_WIDTH * GRID_HEIGHT] =
            [true; GRID_HEIGHT * GRID_HEIGHT];
        new_generation
            .iter_mut()
            .enumerate()
            .for_each(|(idx, cell)| {
                *cell = match self.count_neighbors(idx) {
                    2 if *cell => true,
                    3 => true,
                    _ => false,
                }
            });

        self.cells = new_generation;
    }

    pub fn organisms(&self) -> Organisms {
        todo!();
    }

    pub fn iter(&self) -> std::slice::Iter<bool> {
        self.cells.iter()
    }

    pub fn count_neighbors<I: Into<Index>>(&self, index: I) -> u32 {
        // The neighbor in the case of a grid is only an alive cell
        let index = index.into();
        index.neighbors().iter().fold(0, |acc, &index| {
            if self.cells[usize::from(index)] {
                acc + 1
            } else {
                acc
            }
        })
    }

    pub fn count_ones(&self) -> usize {
        self.cells
            .iter()
            .fold(0, |acc, &cell| if cell { acc + 1 } else { acc })
    }

    pub fn change_cell<I: Into<usize>>(&mut self, index: I) -> Result<()> {
        let index = index.into();

        let cell = self
            .cells
            .get_mut(index)
            .ok_or_else(|| GameError::IndexOutOfBounds(index.into()))?;

        *cell = !*cell;

        Ok(())
    }

    pub fn set_cell<I: Into<usize>>(&mut self, index: I, value: bool) -> Result<()> {
        let index = index.into();

        let cell = self
            .cells
            .get_mut(index)
            .ok_or_else(|| GameError::IndexOutOfBounds(index.into()))?;

        *cell = value;

        Ok(())
    }
}

#[cfg(test)]
impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        !self
            .cells
            .iter()
            .zip(other.cells.iter())
            .any(|(s, o)| s != o)
    }
}

#[cfg(test)]
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cells.iter().collect::<Vec<&bool>>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_all_trues() {
        let grid = Grid::random();
        let true_count = grid
            .iter()
            .fold(0, |acc, &is_true| if is_true { acc + 1 } else { acc });
        assert_ne!(true_count, GRID_HEIGHT * GRID_HEIGHT);
    }

    #[test]
    fn board_changes() {
        let mut grid = Grid::random();

        let old_grid = grid.clone();
        grid.next_gen();
        assert_ne!(grid, old_grid);
        assert_ne!(grid.count_ones(), 0);

        let old_grid = grid.clone();
        grid.next_gen();
        assert_ne!(grid, old_grid);
        assert_ne!(grid.count_ones(), 0);

        let old_grid = grid.clone();
        grid.next_gen();
        assert_ne!(grid, old_grid);
        assert_ne!(grid.count_ones(), 0);

        let old_grid = grid.clone();
        grid.next_gen();
        assert_ne!(grid, old_grid);
        assert_ne!(grid.count_ones(), 0);
    }
}
