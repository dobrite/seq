pub use self::{
    components::{Density, Frac, Length, Prob, Pwm, Rate, Rng},
    config::Config,
    euclid::Euclid,
    gate::Gate,
    output_type::OutputType,
};

mod components;
mod config;
mod euclid;
mod gate;
mod output_type;

pub enum Output {
    Gate(Gate),
    Euclid(Euclid),
}

impl Output {
    pub fn set_prob(&mut self, prob: Prob) {
        match self {
            Output::Gate(gate) => gate.set_prob(prob),
            Output::Euclid(_) => unreachable!(),
        }
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        match self {
            Output::Gate(gate) => gate.set_pwm(pwm),
            Output::Euclid(_) => unreachable!(),
        }
    }

    pub fn set_rate(&mut self, rate: Rate) {
        match self {
            Output::Gate(gate) => gate.set_rate(rate),
            Output::Euclid(euclid) => euclid.set_rate(rate),
        }
    }

    pub fn tick(&mut self, count: u32) {
        match self {
            Output::Gate(gate) => gate.tick(count),
            Output::Euclid(euclid) => euclid.tick(count),
        }
    }
}
