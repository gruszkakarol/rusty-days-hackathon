/// size of a single cell
pub const CELL_SIZE: usize = 20;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub alive: bool,
    pub just_changed: bool,
}

impl Cell {
    pub fn color(&self) -> &str {
        if self.alive {
            "#fff"
        } else {
            "#000"
        }
    }
}

impl Into<Cell> for bool {
    fn into(self) -> Cell {
        Cell {
            alive: self,
            just_changed: false,
        }
    }
}

impl From<Cell> for bool {
    fn from(cell: Cell) -> Self {
        cell.alive
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.alive == other.alive
    }
}
