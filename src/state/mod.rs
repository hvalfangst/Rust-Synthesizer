use std::time::Duration;

use minifb::Key;

use crate::graphics::constants::{WAVEFORM_SINE, WAVEFORM_SQUARE};
use crate::music_theory::{OCTAVE_LOWER_BOUND, OCTAVE_UPPER_BOUND};
use crate::music_theory::note::Note;
use crate::waveforms::Waveform;

pub mod event_loop;
mod utils;

const FRAME_DURATION: Duration = Duration::from_millis(16); // Approximately 60Hz refresh rate

// Synthesizer State Struct
pub struct State {
    octave: i32,
    waveform: Waveform,
    pressed_key: Option<(Key, Note)>,
    waveform_sprite_index: usize,
    filter_factor: f32,
    lpf_active: usize
}

// Initialize Synthesizer State
impl State {
    pub(crate) fn new() -> Self {
        State {
            octave: 4, // Set default octave to 4
            waveform: Waveform::SINE, // Set default waveform to Sine
            pressed_key: None, // Default is no key
            waveform_sprite_index: WAVEFORM_SINE, // Set default waveform sprite index to Sine
            filter_factor: 1.0, // Set default cutoff to 1.0
            lpf_active: 0, // Default for LPF is deactivated
        }
    }

    /// Multiplies the sample frequency with that of the filter cutoff coefficient
    pub fn apply_lpf(&mut self, sample: f32) -> f32 {
        sample * self.filter_factor
    }

    /// Increases the octave by one step, ensuring it does not exceed the upper bound.
    pub fn increase_octave(&mut self) {
        if self.octave < OCTAVE_UPPER_BOUND {
            self.octave += 1;
        }
    }

    /// Decreases the octave by one step, ensuring it does not go below the lower bound.
    pub fn decrease_octave(&mut self) {
        if self.octave > OCTAVE_LOWER_BOUND {
            self.octave -= 1;
        }
    }

    /// Toggle LPF on/off
    pub fn toggle_lpf(&mut self) {
        self.lpf_active ^= 1;
        self.filter_factor = 1.0;
    }

    /// Increases the filter cutoff
    pub fn increase_filter_cutoff(&mut self) {
        if self.lpf_active == 1 && self.filter_factor <= 0.9 {
            self.filter_factor += 0.142857;
        }
    }

    /// Decreases the filter cutoff
    pub fn decrease_filter_cutoff(&mut self) {
        if self.lpf_active == 1 && self.filter_factor >= 0.15 {
            self.filter_factor -= 0.142857;
        }
    }

    /// Returns the current octave value.
    pub fn get_current_octave(&self) -> i32 {
        self.octave
    }

    /// Toggles the waveform between SINE and SQUARE and sets the associated sprite index accordingly.
    pub fn toggle_waveform(&mut self) {
        self.waveform = match self.waveform {
            Waveform::SINE => {
                self.waveform_sprite_index = WAVEFORM_SQUARE;
                Waveform::SQUARE
            },
            Waveform::SQUARE => {
                self.waveform_sprite_index = WAVEFORM_SINE;
                Waveform::SINE
            },
        };
    }
}