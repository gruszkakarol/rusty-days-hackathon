use crate::conway::{SubgridValuesIter, NUMBER_OF_SUBGRIDS};
use crate::soundgen::{Result as SoundResult, SoundGenerator};

pub trait SoundPlayer {
    fn play_sounds(
        pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()>;
}

/// Plays sound in cascading pattern:
/// 1 1   1    1      1         1
///     2   2    2      2         2          2
///           3    3      3         3          3
///                  4      4         4          4
///                           5         5          5
///                                       6          7
/// =====================================================>
/// etc.
pub struct OverlappingCascadePlayer;

/// Plays sounds in reverse cascading pattern
/// 6 6   6    6      6         6
///     5   5    5      5         5          5
///           4    4      4         4          4
///                  3      3         3          3
///                           2         2          2
///                                       1          1
/// =====================================================>
/// etc.
pub struct ReverseOverlappingCascadePlayer;

/// Plays sounds in linear order:
/// 1 1 1 1 1 1 1 1
///                 2 2 2 2 2 2 2 2 2 2
///                                     3 3 3 3 3 3 3 3 3
/// =====================================================>
/// etc.
pub struct LinearPlayer;

/// Plays the sounds in reverse linear order
/// 6 6 6 6 6 6 6 6
///                 5 5 5 5 5 5 5 5 5 5
///                                     4 4 4 4 4 4 4 4 4
/// =====================================================>
/// etc.
pub struct ReverseLinearPlayer;

/// Plays sounds in ping-pong order:
/// 1                 1 1                 1
///   2             2     2             2
///     3         3         3         3
///       4     4             4     4
///         5 5                 5 5
/// ====================================================>
/// etc.
pub struct PingPongPlayer;

/// Plays sounds in ping-pong order:
/// 5                 5 5                 1
///   4             4     4             4
///     3         3         3         3
///       2     2             2     2
///         1 1                 1 1
/// ====================================================>
/// etc.
pub struct ReversePingPongPlayer;

/// Plays sounds in cascading order:
/// 1         1         1
///   2         2         2
///     3         3         3
///       4         4         4
///         5         5         5
/// ====================================================>
/// etc.
pub struct CascadePlayer;

/// Plays sounds in cascading order:
/// 5         5         5
///   4         4         4
///     3         3         3
///       2         2         2
///         1         1         1
/// ====================================================>
/// etc.
pub struct ReverseCascadePlayer;

impl SoundPlayer for OverlappingCascadePlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        let number_of_grids = pitches_and_volumes.len();
        let mut finished_grids_counter: usize = 0;

        for i in 0..number_of_grids * NUMBER_OF_SUBGRIDS {
            let upper_bound: usize = (i + 1).min(number_of_grids);
            for grid_idx in finished_grids_counter..upper_bound {
                match pitches_and_volumes[grid_idx].next() {
                    Some(&(pitch, _)) => {
                        generator.play(pitch)?;
                        sleep();
                    }
                    None => {
                        finished_grids_counter += 1;
                    }
                }
            }
        }

        Ok(())
    }
}

impl SoundPlayer for LinearPlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        pitches_and_volumes
            .iter_mut()
            .flatten()
            .map(|&(pitch, _)| {
                generator.play(pitch)?;
                sleep();
                Ok(())
            })
            .collect::<SoundResult<Vec<_>>>()?;
        Ok(())
    }
}

impl SoundPlayer for ReverseOverlappingCascadePlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        let number_of_grids = pitches_and_volumes.len();
        let mut finished_grids_counter: usize = 0;

        for i in 0..number_of_grids * NUMBER_OF_SUBGRIDS {
            let upper_bound: usize = (i + 1).min(number_of_grids);
            for grid_idx in finished_grids_counter..upper_bound {
                match pitches_and_volumes[number_of_grids - grid_idx].next() {
                    Some(&(pitch, _)) => {
                        generator.play(pitch)?;
                        sleep();
                    }
                    None => {
                        finished_grids_counter += 1;
                    }
                }
            }
        }

        Ok(())
    }
}

impl SoundPlayer for ReverseLinearPlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        pitches_and_volumes
            .iter_mut()
            .flatten()
            .rev()
            .map(|&(pitch, _)| {
                generator.play(pitch)?;
                sleep();
                Ok(())
            })
            .collect::<SoundResult<Vec<_>>>()?;
        Ok(())
    }
}

impl SoundPlayer for PingPongPlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        let number_of_grids = pitches_and_volumes.len();
        let mut direction: isize = 1;
        let mut index: usize = 0;

        for _ in 0..NUMBER_OF_SUBGRIDS {
            for _ in 0..number_of_grids {
                generator.play(
                    pitches_and_volumes[index]
                        .next()
                        .expect("Thish souldn't panic")
                        .0,
                )?;
                sleep();
                index = (index as isize + direction) as usize;
            }
            direction *= -1;
        }

        Ok(())
    }
}

impl SoundPlayer for ReversePingPongPlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        let number_of_grids = pitches_and_volumes.len();
        let mut direction: isize = -1;
        let mut index: usize = number_of_grids - 1;

        for _ in 0..NUMBER_OF_SUBGRIDS {
            for _ in 0..number_of_grids {
                generator.play(
                    pitches_and_volumes[index]
                        .next()
                        .expect("Thish souldn't panic")
                        .0,
                )?;
                sleep();
                index = (index as isize + direction) as usize;
            }
            direction *= -1;
        }

        Ok(())
    }
}

impl SoundPlayer for CascadePlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        let number_of_grids = pitches_and_volumes.len();
        for _ in 0..NUMBER_OF_SUBGRIDS {
            pitches_and_volumes.iter_mut().for_each(|mut iter| {
                generator.play(iter.next().expect("This shouldn't panic").0);
                sleep();
            })
        }

        Ok(())
    }
}

impl SoundPlayer for ReverseCascadePlayer {
    fn play_sounds(
        mut pitches_and_volumes: Vec<SubgridValuesIter>,
        generator: &SoundGenerator,
    ) -> SoundResult<()> {
        let number_of_grids = pitches_and_volumes.len();
        for _ in 0..NUMBER_OF_SUBGRIDS {
            pitches_and_volumes.iter_mut().rev().for_each(|mut iter| {
                generator.play(iter.next().expect("This shouldn't panic").0);
                sleep();
            })
        }

        Ok(())
    }
}

fn sleep() {}
