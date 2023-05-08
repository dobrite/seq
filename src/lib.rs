#![cfg_attr(not(test), no_std)]

use heapless::Vec;

pub use crate::seq::Seq;
pub use lane::{Prob, Pwm, Rate};

mod lane;
mod seq;

pub type LaneStates = Vec<LaneState, 4>;

#[derive(Debug, PartialEq)]
pub struct LaneState {
    pub on: bool,
    pub edge_change: bool,
}
