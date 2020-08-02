//! A simple utility to deterministically generate and play consonant sounds based on
//! some input value.

use thiserror::Error;
use wasm_bindgen::JsValue;
use web_sys::{AudioContext, OscillatorType};

pub type Result<V> = std::result::Result<V, SoundError>;

pub struct SoundGenerator {
    ctx: AudioContext,
    sound: Sound,
    notegen: NoteGenerator,
}

impl SoundGenerator {
    pub fn new() -> SoundGenerator {
        SoundGenerator {
            ctx: AudioContext::new().unwrap(),
            sound: Sound::staccato_sine(),
            notegen: NoteGenerator::new(Range::new(3, 7), Scale::pentatonic()),
        }
    }

    #[allow(dead_code)]
    pub fn set_sound(&mut self, sound: Sound) {
        self.sound = sound;
    }

    #[allow(dead_code)]
    pub fn set_range(&mut self, range: Range) {
        self.notegen.range = range;
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: Scale) {
        self.notegen.scale = scale;
    }

    pub fn play(&self, value: u32) -> Result<()> {
        let sound = &self.sound;

        // Create our web audio objects.
        let primary = self.ctx.create_oscillator()?;
        let fm_osc = self.ctx.create_oscillator()?;
        let gain = self.ctx.create_gain()?;
        let fm_gain = self.ctx.create_gain()?;

        // Some initial settings:
        primary.set_type(sound.oscillator_type);
        primary
            .frequency()
            .set_value(self.notegen.frequency_from_value(value));

        // Give the amp a shape.
        gain.gain().set_value(0.0);
        gain.gain()
            .linear_ramp_to_value_at_time(sound.peak, self.ctx.current_time() + sound.attack)?;
        gain.gain().linear_ramp_to_value_at_time(
            0.0,
            self.ctx.current_time() + sound.sweep - sound.release,
        )?;

        // The FM oscillator isn't really used right now.
        fm_gain.gain().set_value(sound.fm_gain); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(sound.fm_freq);

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
        primary.stop_with_when(self.ctx.current_time() + sound.sweep)?;
        fm_osc.start()?;
        fm_osc.stop_with_when(self.ctx.current_time() + sound.sweep)?;

        Ok(())
    }
}

pub struct Sound {
    attack: f64,
    release: f64,
    sweep: f64,
    peak: f32,
    oscillator_type: OscillatorType,
    fm_gain: f32,
    fm_freq: f32,
}

impl Sound {
    // Constructors
    fn staccato_sine() -> Self {
        Sound {
            attack: 0.15,
            release: 0.3,
            sweep: 0.7,
            peak: 0.6,
            oscillator_type: OscillatorType::Sine,
            fm_gain: 0.0,
            fm_freq: 0.0,
        }
    }
}

pub struct NoteGenerator {
    range: Range,
    scale: Scale,
}

impl NoteGenerator {
    pub fn new(range: Range, scale: Scale) -> Self {
        NoteGenerator { range, scale }
    }

    #[allow(dead_code)]
    pub fn set_range(&mut self, range: Range) {
        self.range = range;
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }

    /// Given a u32 value, this will deterministically produce notes from this generator's scale
    /// and range. The result is a midi note.
    pub fn midi_note_from_value(&self, value: u32) -> u8 {
        let divisor = self.scale.len();

        // Choose a note from the scale.
        let base = self.scale.0[value as usize % divisor] as u8;

        // Figure out how many octaves we have to move it up.
        let octave_shift = (value / divisor as u32) % self.range.count();

        12 + base + ((self.range.lowest_octave as u32 + octave_shift) * 12) as u8
    }

    pub fn frequency_from_value(&self, value: u32) -> f32 {
        midi_to_freq(self.midi_note_from_value(value))
    }
}

// Valid octaves: 1-8 - these are the octaves from music theory
pub struct Range {
    lowest_octave: u8,
    highest_octave: u8,
}

impl Range {
    pub fn new(lowest_octave: u8, highest_octave: u8) -> Self {
        Range {
            lowest_octave,
            highest_octave,
        }
    }

    pub fn count(&self) -> u32 {
        (self.highest_octave - self.lowest_octave + 1).into()
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum Note {
    C = 0,
    Db = 1,
    D = 2,
    Eb = 3,
    E = 4,
    F = 5,
    Gb = 6,
    G = 7,
    Ab = 8,
    A = 9,
    Bb = 10,
    B = 11,
}

pub struct Scale(Vec<Note>);

impl Scale {
    pub fn pentatonic() -> Self {
        use Note::*;

        Scale(vec![C, D, E, G, A])
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

#[test]
fn note_generation_single_octave_pentatonic() {
    let notegen = NoteGenerator::new(Range::new(1, 1), Scale::pentatonic());

    assert_eq!(notegen.midi_note_from_value(0), 24);
    assert_eq!(notegen.midi_note_from_value(1), 26);
    assert_eq!(notegen.midi_note_from_value(2), 28);
    assert_eq!(notegen.midi_note_from_value(3), 31);
    assert_eq!(notegen.midi_note_from_value(4), 33);
    assert_eq!(notegen.midi_note_from_value(5), 24);
}

#[test]
fn note_generation_three_octave_pentatonic() {
    let notegen = NoteGenerator::new(Range::new(2, 4), Scale::pentatonic());

    assert_eq!(notegen.midi_note_from_value(1), 38);
    assert_eq!(notegen.midi_note_from_value(2), 40);
    assert_eq!(notegen.midi_note_from_value(3), 43);
    assert_eq!(notegen.midi_note_from_value(4), 45);
    assert_eq!(notegen.midi_note_from_value(5), 48);
    assert_eq!(notegen.midi_note_from_value(8), 55);
    assert_eq!(notegen.midi_note_from_value(13), 67);
    assert_eq!(notegen.midi_note_from_value(18), 43);
}
