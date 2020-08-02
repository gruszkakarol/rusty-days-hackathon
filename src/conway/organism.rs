use super::grid::Grid;

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
pub struct Organisms<'o> {
    grid: &'o Grid,
}

impl<'o> Iterator for Organisms<'o> {
    type Item = &'o Organism;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
