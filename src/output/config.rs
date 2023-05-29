use heapless::Vec;

use super::{Density, Length, OutputType, Prob, Pwm, Rate};

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub density: Density,
    pub length: Length,
    pub output_type: OutputType,
    pub prob: Prob,
    pub pwm: Pwm,
    pub rate: Rate,
    pub sequence: Vec<bool, 16>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let mut sequence = Vec::new();
        sequence.resize_default(16).ok();

        Self {
            density: Density(4),
            length: Length(16),
            output_type: OutputType::Gate,
            prob: Prob::P100,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            sequence,
        }
    }
}
