use rand::prelude::Rng;
use rand::rngs::OsRng;

use super::cell::Cell;
use super::index::Index;
use super::GameError;
use super::Result;

/// width of a single grid
pub const GRID_WIDTH: usize = 25;
/// height of a single grid
pub const GRID_HEIGHT: usize = 25;

pub const NUMBER_OF_SUBGRIDS: usize = 1;

/// Iterator over the values of pitch and volume for each subgrid in the Grid
pub type SubgridValuesIter<'g> = std::slice::Iter<'g, (u32, u32)>;

#[derive(Clone)]
pub struct Grid {
    sound: u32,
    pub stopped: bool,
    cells: [Cell; GRID_WIDTH * GRID_HEIGHT],
    subgrids: [(Index, Index); NUMBER_OF_SUBGRIDS],
    subgrid_values: [(u32, u32); NUMBER_OF_SUBGRIDS],
}

impl Grid {
    pub fn new(cells: [Cell; GRID_WIDTH * GRID_HEIGHT], sound: u32, stopped: bool) -> Self {
        Self {
            cells,
            sound,
            stopped,
            subgrids: Self::subgrids(),
            subgrid_values: Default::default(),
        }
    }

    pub fn random() -> Grid {
        let mut cells: [Cell; GRID_WIDTH * GRID_HEIGHT] = [false.into(); GRID_WIDTH * GRID_HEIGHT];

        cells.iter_mut().for_each(|cell| {
            let value = OsRng.gen_bool(1.2 / 3.0).into();
            // println!("{:?}", value);
            *cell = value;
        });

        Self {
            cells,
            stopped: false,
            sound: Default::default(),
            subgrids: Self::subgrids(),
            subgrid_values: Default::default(),
        }
    }

    pub fn next_gen(&mut self) -> bool {
        if self.stopped {
            return false;
        }

        let mut new_generation: [Cell; GRID_WIDTH * GRID_HEIGHT] =
            [true.into(); GRID_HEIGHT * GRID_HEIGHT];
        new_generation
            .iter_mut()
            .enumerate()
            .for_each(|(idx, cell)| {
                let cell_alive = self.cells[idx].alive;
                let neighbors = self.count_neighbors(idx);
                *cell = match neighbors {
                    2 if cell_alive => Cell {
                        alive: true,
                        just_changed: false,
                    },
                    3 => Cell {
                        alive: true,
                        just_changed: !cell_alive,
                    },
                    _ => Cell {
                        alive: false,
                        just_changed: cell_alive,
                    },
                };
            });

        self.cells = new_generation;

        true
    }

    pub fn iter(&self) -> std::slice::Iter<Cell> {
        self.cells.iter()
    }

    pub fn count_neighbors<I: Into<Index>>(&self, index: I) -> usize {
        // The neighbor in the case of a grid is only an alive cell
        let index = index.into();
        let mut neighbor_counter: usize = 0;

        index.neighbors().iter().for_each(|neighbor_idx| {
            neighbor_counter += self.cells[usize::from(*neighbor_idx)].alive as usize
        });

        neighbor_counter
    }

    pub fn count_ones(&self) -> usize {
        let mut one_counter: usize = 0;

        self.cells
            .iter()
            .for_each(|cell| one_counter += cell.alive as usize);

        one_counter
    }

    pub fn change_cell<I: Into<usize>>(&mut self, index: I) -> Result<()> {
        let index = index.into();

        let cell = self
            .cells
            .get_mut(index)
            .ok_or_else(|| GameError::IndexOutOfBounds(index.into()))?;

        *cell = Cell {
            alive: !cell.alive,
            just_changed: true,
        };

        Ok(())
    }

    pub fn get_cell<I: Into<usize>>(&self, index: I) -> Option<&Cell> {
        let index = index.into();
        self.cells.get(index)
    }

    pub fn set_cell<I: Into<usize>>(&mut self, index: I, value: bool) -> Result<()> {
        let index = index.into();

        let cell = self
            .cells
            .get_mut(index)
            .ok_or_else(|| GameError::IndexOutOfBounds(index.into()))?;

        *cell = Cell {
            alive: value,
            just_changed: value != cell.alive,
        };

        Ok(())
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn start(&mut self) {
        self.stopped = false;
    }

    pub fn toggle(&mut self) {
        self.stopped = !self.stopped
    }

    /// Counts the deaths in each subgrid.
    /// Returns counted values for each subgrid
    /// Returns (pitch, volume)
    pub fn get_pitch_and_volume_per_subgrid(&mut self) -> SubgridValuesIter {
        for (subgrid_idx, (index_start, index_end)) in self.subgrids.iter().enumerate() {
            let mut pitch_value: u32 = 0;
            let mut volume_value: u32 = 0;

            for row in index_start.row..index_end.row {
                for col in index_start.col..index_end.col {
                    let idx = row * GRID_WIDTH + col;
                    if self.cells[idx].alive {
                        volume_value += self.cells[idx].just_changed as u32;
                    } else {
                        pitch_value += self.cells[idx].just_changed as u32;
                    }
                }
            }

            self.subgrid_values[subgrid_idx] = (pitch_value, volume_value);
        }
        self.subgrid_values.iter()
    }

    /// Counts cells that have died in last iteration and cells that has been raise in the
    /// last iteration.
    /// Returns (pitch, volume)
    pub fn get_pitch_and_volume(&self) -> (u32, u32) {
        let mut pitch_value: u32 = 0;
        let mut volume_value: u32 = 0;

        self.cells.iter().for_each(|cell| {
            let just_changed = cell.just_changed as u32;
            pitch_value += !cell.alive as u32 * just_changed;
            volume_value += cell.alive as u32 * just_changed;
        });

        (pitch_value, volume_value)
    }

    /// Returns subgrids for the current grid
    fn subgrids() -> [(Index, Index); NUMBER_OF_SUBGRIDS] {
        let mut subgrids: [(Index, Index); NUMBER_OF_SUBGRIDS] =
            [(0usize.into(), 0usize.into()); NUMBER_OF_SUBGRIDS];
        let n = (NUMBER_OF_SUBGRIDS as f32).sqrt() as usize;
        let subgrid_rows_number: usize = GRID_HEIGHT / n;
        let subgrid_cols_number: usize = GRID_WIDTH / n;

        for c in 0..n {
            for r in 0..n {
                let index_start = Index {
                    row: r * subgrid_rows_number,
                    col: c * subgrid_cols_number,
                };

                let index_end = Index {
                    row: (r + 1) * subgrid_rows_number - 1,
                    col: (c + 1) * subgrid_cols_number - 1,
                };

                subgrids[c * n + r] = (index_start, index_end);
            }
        }

        subgrids
    }
}

#[cfg(test)]
mod test {
    impl PartialEq for Grid {
        fn eq(&self, other: &Self) -> bool {
            self.cells
                .iter()
                .zip(other.cells.iter())
                .all(|(s, o)| s == o)
        }
    }

    // NOTE: This implementation is highly unoptimized, but it is needed for testing purposes only,
    // so it will not impact the performance of the application
    impl std::fmt::Debug for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.cells.iter().collect::<Vec<&Cell>>())
        }
    }

    use super::*;

    #[test]
    fn not_all_trues() {
        let grid = Grid::random();
        let true_count = grid
            .iter()
            .fold(0, |acc, &cell| if cell.alive { acc + 1 } else { acc });
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
    use std::time::Instant;

    #[test]
    fn some_cells_died() {
        let mut grid = Grid::random();
        let now = Instant::now();
        grid.next_gen();
        let mut deaths_in_subgrids = grid.get_pitch_and_volume_per_subgrid();
        let now = now.elapsed().as_secs_f32();
        println!("{}", now);
        assert!(!deaths_in_subgrids.any(|&(pitch, volume)| pitch == 0 && volume == 0));
    }
}
