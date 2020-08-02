//! A simple utility to deterministically generate and play consonant sounds based on
//! some input value.

use thiserror::Error;

type Result<V> = std::result::Result<V, SoundError>;

pub struct SoundGenerator;

impl SoundGenerator {
    pub fn new() -> SoundGenerator {
        SoundGenerator
    }

    pub fn play(&self, value: u32) -> Result<()> {
        todo!();
    }
}

// For how to use this, see: https://docs.rs/thiserror/1.0.20/thiserror/
#[derive(Error, Debug)]
pub enum SoundError {
    
}