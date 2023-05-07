#![cfg_attr(not(test), no_std)]

use heapless::Vec;

pub use outputs::Outputs;
pub use prob::Prob;
pub use pwm::Pwm;
pub use rate::Rate;

mod output;
mod outputs;
mod prob;
mod pwm;
mod rate;
mod rng;

#[derive(Debug, PartialEq)]
pub struct OutputStates {
    pub outputs: Vec<OutputState, 4>,
}

#[derive(Debug, PartialEq)]
pub struct OutputState {
    pub on: bool,
    pub edge_change: bool,
}
