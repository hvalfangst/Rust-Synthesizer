use rodio::Source;
use std::{f32::consts::PI, time::Duration};
use crate::{
    effects::low_pass_filter::LowPassFilter,
    waveforms::{MONO, SAMPLE_RATE}
};

#[derive(Debug)]
pub struct SineWave {
    freq: f32,
    num_sample: usize,
    pub filter: LowPassFilter
}

impl SineWave {
    pub fn new(freq: f32) -> SineWave {
        SineWave { freq, num_sample: 0, filter: LowPassFilter::new() }
    }
    pub fn generate_sine_wave(&mut self) -> f32 {
        calculate_sine(self.freq, self.num_sample)
    }
}

/// Implementation of the `Iterator` trait for the `SineWave`.
impl Iterator for SineWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        // increment sample counter by 1
        self.num_sample = self.num_sample.wrapping_add(1);
        let sine_wave = self.generate_sine_wave();

        // Only apply low-pass filter if it is indeed active
        if self.filter.filter_active() {
            self.filter.filtered_value = sine_wave * self.filter.low_pass_filter();
            Some(self.filter.filtered_value)
        } else {
            Some(sine_wave)
        }
    }
}

/// Implementation of the `Source` trait for the `SineWave`.
impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        MONO
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

///  The formula for calculating a sine wave is 'y(t) = sin(2πft)', whereby:
/// '2πf' is two times pi times the frequency (ie 440 hz)
/// 't' is time in seconds in relation to the sample rate (ie 1/48k)
pub fn calculate_sine(frequency: f32, num_sample: usize) -> f32 {
    // Calculate time in seconds based on the sample number and the sample rate
    let time: f32 = num_sample as f32 / SAMPLE_RATE;

    // Calculate angular frequency (2πf)
    let angular_frequency: f32 = 2.0 * PI * frequency;

    // Calculate the sine wave value using the angular frequency and time
    (angular_frequency * time).sin()
}