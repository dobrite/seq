#![cfg_attr(not(test), no_std)]

use heapless::Vec;

mod output;
mod outputs;
mod prob;
mod pwm;
mod rate;
mod rng;

pub use output::{Output, State};
pub use outputs::Outputs;
pub use prob::Prob;
pub use pwm::Pwm;
pub use rate::Rate;

#[derive(Debug, PartialEq)]
pub struct OutputState {
    pub outputs: Vec<State, 4>,
}

#[derive(Debug, PartialEq)]
pub struct Tick {
    pub major: bool,
}
