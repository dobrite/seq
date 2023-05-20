use super::{Prob, Pwm, Rate};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub prob: Prob,
    pub pwm: Pwm,
    pub rate: Rate,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        State {
            prob: Prob::P100,
            pwm: Pwm::P50,
            rate: Rate::Unity,
        }
    }
}
