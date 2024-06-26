use std::fmt;

pub struct Octave {
    pub value: i32
}

/// Enumerates musical notes A, B, C, D, E, F, and G.
#[derive(Debug)]
pub enum Note {
    A, B, C, D, E, F, G,
}


/// Implements the [Display] trait for [Note]
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Note::A => write!(f, "A"),
            Note::B => write!(f, "B"),
            Note::C => write!(f, "C"),
            Note::D => write!(f, "D"),
            Note::E => write!(f, "E"),
            Note::F => write!(f, "F"),
            _ => write!(f, "G")
        }
    }
}

impl Note {
    /// Computes the frequency of the note.rs based on the following: [frequency * (2^(octave-4))].
    ///
    /// # Arguments
    ///
    /// * `octave` - The current octave.
    ///
    /// # Returns
    ///
    /// The adjusted frequency of the note.rs based on the current octave.
    pub fn frequency(&self, octave: &Octave) -> f32 {
        let base_frequency = match self {
            Note::A => 440.0,
            Note::B => 493.88,
            Note::C => 523.25,
            Note::D => 587.33,
            Note::E => 659.25,
            Note::F => 698.46,
            Note::G => 783.99,
        };

        // Adjust the base frequency based on the current octave setting
        base_frequency * 2.0_f32.powi(octave.value - 4)
    }
}