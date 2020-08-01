#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub alive: bool,
    pub just_killed: bool,
}

impl Into<Cell> for bool {
    fn into(self) -> Cell {
        Cell {
            alive: self,
            just_killed: false,
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
