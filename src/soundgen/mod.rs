//! A simple utility to deterministically generate and play consonant sounds based on
//! some input value.

use thiserror::Error;
use web_sys::{AudioContext, OscillatorType};
use wasm_bindgen::JsValue;

type Result<V> = std::result::Result<V, SoundError>;

const ATTACK: f64 = 0.15;
const RELEASE: f64 = 0.3;
const SWEEP: f64 = 0.7;
const PEAK: f32 = 0.6;

pub struct SoundGenerator {
    ctx: AudioContext,
}

impl SoundGenerator {
    pub fn new() -> SoundGenerator {
        SoundGenerator {
            ctx: AudioContext::new().unwrap(),
        }
    }

    pub fn play(&self, _value: u32) -> Result<()> {
        // Create our web audio objects.
        let primary = self.ctx.create_oscillator()?;
        let fm_osc = self.ctx.create_oscillator()?;
        let gain = self.ctx.create_gain()?;
        let fm_gain = self.ctx.create_gain()?;

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(midi_to_freq(66));

        // Give the amp a shape.
        gain.gain().set_value(0.0);
        gain.gain().linear_ramp_to_value_at_time(PEAK, self.ctx.current_time() + ATTACK)?;
        gain.gain().linear_ramp_to_value_at_time(0.0, self.ctx.current_time() + SWEEP - RELEASE)?;
        
        // The FM oscillator isn't really used right now.
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);

        // Connect the nodes up!

        // The primary oscillator is routed through the gain node, so that
        // it can control the overall output volume.
        primary.connect_with_audio_node(&gain)?;

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&self.ctx.destination())?;

        // The FM oscillator is connected to its own gain node, so it can
        // control the amount of modulation.
        fm_osc.connect_with_audio_node(&fm_gain)?;

        // Connect the FM oscillator to the frequency parameter of the main
        // oscillator, so that the FM node can modulate its frequency.
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        primary.stop_with_when(self.ctx.current_time() + SWEEP)?;
        fm_osc.start()?;
        fm_osc.stop_with_when(self.ctx.current_time() + SWEEP)?;

        Ok(())
    }
}

fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

// For how to use this, see: https://docs.rs/thiserror/1.0.20/thiserror/
#[derive(Error, Debug)]
pub enum SoundError {
    #[error("WASM error")]
    WasmError,
}

impl From<JsValue> for SoundError {
    fn from(_v: JsValue) -> Self {
        // TODO: Figure out how to represent this JS value in a "sendable" way.
        SoundError::WasmError
    }
}