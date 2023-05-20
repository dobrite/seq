pub use self::{
    components::{Frac, Prob, Pwm, Rate, Rng},
    euclid::Euclid,
    gate::{Gate, State},
};

mod components;
mod euclid;
mod gate;

pub enum Lane {
    Gate(Gate),
    Euclid(Euclid),
}

impl Lane {
    pub fn set_prob(&mut self, prob: Prob) {
        match self {
            Lane::Gate(gate) => gate.set_prob(prob),
            Lane::Euclid(_) => unreachable!(),
        }
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        match self {
            Lane::Gate(gate) => gate.set_pwm(pwm),
            Lane::Euclid(_) => unreachable!(),
        }
    }

    pub fn set_rate(&mut self, rate: Rate) {
        match self {
            Lane::Gate(gate) => gate.set_rate(rate),
            Lane::Euclid(euclid) => euclid.set_rate(rate),
        }
    }

    pub fn tick(&mut self, count: u32) {
        match self {
            Lane::Gate(gate) => gate.tick(count),
            Lane::Euclid(euclid) => euclid.tick(count),
        }
    }
}
