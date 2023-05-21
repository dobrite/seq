use super::{Prob, Pwm, Rate};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub prob: Prob,
    pub pwm: Pwm,
    pub rate: Rate,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            prob: Prob::P100,
            pwm: Pwm::P50,
            rate: Rate::Unity,
        }
    }
}
