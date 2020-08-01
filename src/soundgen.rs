//! A simple utility to deterministically generate and play consonant sounds based on
//! some input value.

use thiserror::Error;

type Result<V> = std::result::Result<V, SoundError>;

struct SoundGenerator;

impl SoundGenerator {
    fn play(value: u32) -> Result<()> {
        todo!();
    }
}

// For how to use this, see: https://docs.rs/thiserror/1.0.20/thiserror/
#[derive(Error, Debug)]
pub enum SoundError {
    
}