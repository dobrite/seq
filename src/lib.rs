#![cfg_attr(not(test), no_std)]

use heapless::Vec;
pub use output::{Config as OutputConfig, Density, Frac, Length, OutputType, Prob, Pwm, Rate};
pub use ticks::tick_duration;

use crate::output::Output;
pub use crate::seq::Seq;

mod output;
mod seq;
mod ticks;

pub type OutputStates = Vec<OutputState, 4>;

#[derive(Debug, PartialEq)]
pub struct OutputState {
    pub on: bool,
    pub edge_change: bool,
}

impl From<&Output> for OutputState {
    fn from(val: &Output) -> Self {
        match val {
            Output::Gate(gate) => OutputState {
                on: gate.on,
                edge_change: gate.edge_change,
            },
            Output::Euclid(euclid) => OutputState {
                on: euclid.on,
                edge_change: euclid.edge_change,
            },
        }
    }
}
