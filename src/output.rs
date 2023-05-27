pub use self::{
    components::{Density, Frac, Length, OutputType, Prob, Pwm, Rate, Rng},
    config::Config,
    gate::Gate,
};

mod components;
mod config;
mod gate;

pub enum Output {
    Gate(Gate),
}

impl Output {
    pub fn set_prob(&mut self, prob: Prob) {
        match self {
            Output::Gate(gate) => gate.set_prob(prob),
        }
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        match self {
            Output::Gate(gate) => gate.set_pwm(pwm),
        }
    }

    pub fn set_rate(&mut self, rate: Rate) {
        match self {
            Output::Gate(gate) => gate.set_rate(rate),
        }
    }

    pub fn set_length(&mut self, length: Length) {
        match self {
            Output::Gate(gate) => gate.set_length(length),
        }
    }

    pub fn set_density(&mut self, density: Density) {
        match self {
            Output::Gate(gate) => gate.set_density(density),
        }
    }

    pub fn tick(&mut self, count: u32) {
        match self {
            Output::Gate(gate) => gate.tick(count),
        }
    }
}
