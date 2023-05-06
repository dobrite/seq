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
pub struct OutputState {
    pub outputs: Vec<bool, 4>,
}
