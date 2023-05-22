use super::{Density, Length, OutputType, Prob, Pwm, Rate};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub density: Density,
    pub length: Length,
    pub prob: Prob,
    pub pwm: Pwm,
    pub rate: Rate,
    pub output_type: OutputType,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            density: Density(4),
            length: Length(16),
            prob: Prob::P100,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            output_type: OutputType::Gate,
        }
    }
}
