#![cfg_attr(not(test), no_std)]

use heapless::Vec;
pub use lane::{Prob, Pwm, Rate};
pub use ticks::tick_duration;

pub use crate::seq::Seq;

mod lane;
mod seq;
mod ticks;

pub type LaneStates = Vec<LaneState, 4>;

#[derive(Debug, PartialEq)]
pub struct LaneState {
    pub on: bool,
    pub edge_change: bool,
}
