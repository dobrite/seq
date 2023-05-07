#![cfg_attr(not(test), no_std)]

use heapless::Vec;

pub use output::{Prob, Pwm, Rate};
pub use outputs::Outputs;

mod output;
mod outputs;

#[derive(Debug, PartialEq)]
pub struct OutputStates {
    pub outputs: Vec<OutputState, 4>,
}

#[derive(Debug, PartialEq)]
pub struct OutputState {
    pub on: bool,
    pub edge_change: bool,
}
