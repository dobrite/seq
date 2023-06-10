use heapless::Vec;

use self::components::MAX_STEPS;
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    density: Density,
    length: Length,
    output_type: OutputType,
    prob: Prob,
    pwm: Pwm,
    rate: Rate,
    sequence: Sequence,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self {
            density: Density(4),
            length: Length(MAX_STEPS as u32),
            output_type: OutputType::Gate,
            prob: Prob::P100,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            sequence: Vec::new(),
        };

        config.set_output_type(config.output_type);

        config
    }

    pub fn density(&self) -> Density {
        self.density
    }

    pub fn length(&self) -> Length {
        self.length
    }

    pub fn output_type(&self) -> OutputType {
        self.output_type
    }

    pub fn prob(&self) -> Prob {
        self.prob
    }

    pub fn pwm(&self) -> Pwm {
        self.pwm
    }

    pub fn rate(&self) -> Rate {
        self.rate
    }

    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }

    pub fn set_sequence(&mut self, length: Length, density: Density) {
        self.length = length;
        self.density = density;
        euclid(self.density, self.length, &mut self.sequence);
    }

    pub fn set_output_type(&mut self, output_type: OutputType) {
        self.output_type = output_type;
        let density = match output_type {
            OutputType::Gate => Density(self.length.0),
            OutputType::Euclid => self.density,
        };
        let prob = match output_type {
            OutputType::Gate => self.prob,
            OutputType::Euclid => Prob::P100,
        };
        self.set_prob(prob);
        euclid(density, self.length, &mut self.sequence);
    }

    pub fn set_prob(&mut self, prob: Prob) {
        self.prob = prob;
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        self.pwm = pwm;
    }

    pub fn set_rate(&mut self, rate: Rate) {
        self.rate = rate;
    }
}
