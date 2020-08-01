use rand::prelude::{thread_rng, Rng};

use super::cell::Cell;
use super::index::Index;
use super::organism::Organisms;
use super::GameError;
use super::Result;

/// width of a single grid
pub const GRID_WIDTH: usize = 500;
/// height of a single grid
pub const GRID_HEIGHT: usize = 500;

pub const NUMBER_OF_SUBGRIDS: usize = 25;

#[derive(Clone)]
pub struct Grid {
    sound: u32,
    stopped: bool,
    cells: [Cell; GRID_WIDTH * GRID_HEIGHT],
    subgrids: Vec<(Index, Index)>,
}

impl Grid {
    pub fn new(cells: [Cell; GRID_WIDTH * GRID_HEIGHT], sound: u32, stopped: bool) -> Self {
        Self {
            cells,
            sound,
            stopped,
            subgrids: Self::subgrids(),
        }
    }

    pub fn random() -> Grid {
        let mut rng = thread_rng();
        let mut cells: [Cell; GRID_WIDTH * GRID_HEIGHT] = [true.into(); GRID_WIDTH * GRID_HEIGHT];

        cells
            .iter_mut()
            .for_each(|cell| *cell = rng.gen_bool(1.2 / 3.0).into());

        Self {
            cells,
            stopped: false,
            sound: Default::default(),
            subgrids: Self::subgrids(),
        }
    }

    pub fn next_gen(&mut self) -> usize {
        if self.stopped {
            return 0;
        }

        let mut death_counter: usize = 0;
        let mut new_generation: [Cell; GRID_WIDTH * GRID_HEIGHT] =
            [true.into(); GRID_HEIGHT * GRID_HEIGHT];
        new_generation
            .iter_mut()
            .enumerate()
            .for_each(|(idx, cell)| {
                *cell = match self.count_neighbors(idx) {
                    2 if bool::from(*cell) => true.into(),
                    3 => true.into(),
                    _ => {
                        death_counter += bool::from(*cell) as usize;
                        Cell {
                            alive: false,
                            just_killed: bool::from(*cell),
                        }
                    }
                }
            });

        self.cells = new_generation;

        death_counter
    }

    pub fn organisms(&self) -> Organisms {
        todo!();
    }

    pub fn iter(&self) -> std::slice::Iter<Cell> {
        self.cells.iter()
    }

    pub fn count_neighbors<I: Into<Index>>(&self, index: I) -> usize {
        // The neighbor in the case of a grid is only an alive cell
        let index = index.into();
        Self::__count_neighbors(&self.cells, index)
    }

    pub fn count_ones(&self) -> usize {
        self.cells
            .iter()
            .fold(0, |acc, &cell| if bool::from(cell) { acc + 1 } else { acc })
    }

    pub fn change_cell<I: Into<usize>>(&mut self, index: I) -> Result<()> {
        let index = index.into();

        let cell = self
            .cells
            .get_mut(index)
            .ok_or_else(|| GameError::IndexOutOfBounds(index.into()))?;

        *cell = Cell {
            alive: !cell.alive,
            just_killed: cell.alive,
        };

        Ok(())
    }

    pub fn set_cell<I: Into<usize>>(&mut self, index: I, value: bool) -> Result<()> {
        let index = index.into();

        let cell = self
            .cells
            .get_mut(index)
            .ok_or_else(|| GameError::IndexOutOfBounds(index.into()))?;

        *cell = Cell {
            alive: value,
            just_killed: !value,
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
    pub fn count_deaths_in_subgrids(&self) -> Vec<u32> {
        let mut subgrid_deaths: Vec<u32> = Vec::with_capacity(NUMBER_OF_SUBGRIDS);

        for (index_start, index_end) in &self.subgrids {
            let mut subgrid_death_counter: u32 = 0;

            for row in index_start.row..index_end.row {
                for col in index_start.col..index_end.col {
                    subgrid_death_counter += self.cells[row * GRID_WIDTH + col].just_killed as u32;
                }
            }

            subgrid_deaths.push(subgrid_death_counter);
        }

        subgrid_deaths
    }

    /// Returns subgrids for the current grid
    fn subgrids() -> Vec<(Index, Index)> {
        let mut subgrids: Vec<(Index, Index)> = Vec::with_capacity(NUMBER_OF_SUBGRIDS);
        let n = (NUMBER_OF_SUBGRIDS as f32).sqrt() as usize;
        let subgrid_rows_number: usize = GRID_HEIGHT / n;
        let subgrid_cols_number: usize = GRID_WIDTH / n;

        let row = 0;
        let col = 0;

        for c in 0..n {
            for r in 0..n {
                let index_start = Index {
                    row: 0 + r * subgrid_rows_number,
                    col: 0 + c * subgrid_cols_number,
                };

                let index_end = Index {
                    row: 0 + (r + 1) * subgrid_rows_number - 1,
                    col: 0 + (c + 1) * subgrid_cols_number - 1,
                };

                subgrids.push((index_start, index_end));
            }
        }

        subgrids
    }

    // fn find_organisms(&self) {
    //     let mut board = self.cells.clone();

    //     for x in 0..GRID_WIDTH {
    //         for y in 0..GRID_HEIGHT {
    //             let index = Index { row: y, col: x };

    //             if !bool::from(board[usize::from(index)]) {
    //                 continue;
    //             }

    //             let organism = Self::crawl_cells(&mut board, index);
    //             Self::get_organism_center(organism);
    //             todo!()
    //         }
    //     }
    // }

    // fn crawl_cells(
    //     board: &mut [Cell; GRID_HEIGHT * GRID_WIDTH],
    //     index: Index,
    // ) -> Vec<(Index, usize)> {
    //     let u_index = usize::from(index);
    //     if !bool::from(board[u_index]) {
    //         return Vec::new();
    //     }

    //     let mut vec: Vec<(Index, usize)> = Vec::new();

    //     vec.push((index, 1 + Self::__count_neighbors(&board, index)));
    //     board[u_index] = Cell {
    //         alive: false,
    //         just_killed: false,
    //     };

    //     for neighbor in index.neighbors() {
    //         let mut crawled_vec = Self::crawl_cells(board, neighbor);
    //         vec.append(&mut crawled_vec);
    //     }

    //     vec
    // }

    // fn get_organism_center(organism: Vec<(Index, usize)>) -> Index {
    //     let mut cells_weight: usize = 0;
    //     let mut row: usize = 0;
    //     let mut col: usize = 0;

    //     organism.iter().for_each(|(idx, worth)| {
    //         cells_weight += worth;
    //         row += idx.row * worth;
    //         col += idx.col * worth;
    //     });

    //     Index {
    //         row: (row as f32 / cells_weight as f32).round() as usize,
    //         col: (col as f32 / cells_weight as f32).round() as usize,
    //     }
    // }

    fn __count_neighbors(board: &[Cell; GRID_HEIGHT * GRID_WIDTH], index: Index) -> usize {
        index.neighbors().iter().fold(0, |acc, &index| {
            if bool::from(board[usize::from(index)]) {
                acc + 1
            } else {
                acc
            }
        })
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
        write!(f, "{:?}", self.cells.iter().collect::<Vec<&Cell>>())
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
            .fold(0, |acc, &cell| if bool::from(cell) { acc + 1 } else { acc });
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
        let deaths_in_subgrids = grid.count_deaths_in_subgrids();
        let now = now.elapsed().as_secs_f32();
        println!("{}", now);
        assert!(!deaths_in_subgrids
            .iter()
            .any(|&number_of_deaths| number_of_deaths == 0));
    }
}
