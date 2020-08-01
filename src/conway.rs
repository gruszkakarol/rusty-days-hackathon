//! This contains the Conway's Game of Life logic. Notably:
//! * generating the next generation from the previous one,
//! * producing an iterator over all the organisms in a generation,
//! * counting the cells of an organism, and
//! * determining whether an organism is dying.