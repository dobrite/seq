use heapless::Vec;

use super::Rng;

pub type OutputStates = Vec<OutputState, 4>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutputState {
    pub index: usize,
    pub index_change: bool,
    pub on: bool,
    pub on_change: bool,
    pub rng: Rng,
}
