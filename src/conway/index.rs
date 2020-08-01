//! The thinking process behind the index
//!                  width
//!       <===========================>
//!       ----------------------------- /\
//!   | 0 |0 1 2 3 4 ...           w-1| ||
//! r | 1 |w w+1   ...                | || h
//! o | 2 |                           | || e
//! w | 3 |                           | || i
//! s | 4 |                           | || g
//!   | 5 |                           | || h
//!   | 6 |                           | || t
//!  \/ 7 |                           | ||
//!       ----------------------------- \/
//!        0 1 2 3 4 5 6 7 8 9 ...
//!        --------------------->
//!               columns

use super::grid::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index {
    pub row: usize,
    pub col: usize,
}

impl Index {
    pub fn neighbors(self) -> Vec<Self> {
        let mut neighbors: Vec<Self> = Vec::with_capacity(8);

        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                if x == 0 && y == 0 {
                    continue;
                }

                let row = self.row as isize + x;

                if row < 0 || row >= GRID_HEIGHT as isize {
                    continue;
                }

                let col = self.col as isize + y;

                if col < 0 || col >= GRID_WIDTH as isize {
                    continue;
                }

                let row = row as usize;
                let col = col as usize;

                neighbors.push(Index { row, col })
            }
        }

        neighbors
    }
}

impl Into<Index> for usize {
    fn into(self) -> Index {
        Index {
            row: self / GRID_WIDTH,
            col: self % GRID_WIDTH,
        }
    }
}

impl From<Index> for usize {
    fn from(index: Index) -> Self {
        index.row * GRID_WIDTH + index.col
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neighbors() {
        let index = Index { row: 1, col: 1 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 8);

        let index = Index { row: 0, col: 1 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 5);

        let index = Index { row: 1, col: 0 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 5);

        let index = Index { row: 0, col: 0 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 3);

        let index = Index { row: 499, col: 1 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 5);

        let index = Index { row: 1, col: 499 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 5);

        let index = Index { row: 499, col: 499 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 3);

        let index = Index { row: 125, col: 420 };
        let neighbors = index.neighbors();
        assert_eq!(neighbors.len(), 8);
    }
}
