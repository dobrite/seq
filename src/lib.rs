#![cfg_attr(not(test), no_std)]

use heapless::Vec;
pub use output::{Config as OutputConfig, Density, Frac, Length, OutputType, Prob, Pwm, Rate};

use crate::output::Output;
pub use crate::seq::Seq;

mod output;
mod seq;
mod tick;

pub type OutputStates = Vec<OutputState, 4>;

#[derive(Debug, PartialEq)]
pub struct OutputState {
    pub on: bool,
    pub edge_change: bool,
}

impl From<&Output> for OutputState {
    fn from(val: &Output) -> Self {
        OutputState {
            on: val.on,
            edge_change: val.edge_change,
        }
    }
}
