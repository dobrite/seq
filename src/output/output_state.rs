use heapless::Vec;

use super::Rng;

pub type OutputStates = Vec<OutputState, 4>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutputState {
    pub index: u32,
    pub index_change: bool,
    pub on: bool,
    pub on_change: bool,
    pub rng: Rng,
}
