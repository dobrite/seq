use heapless::Vec;

use super::Rng;

pub type OutputStates = Vec<OutputState, 4>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutputState {
    pub on: bool,
    pub edge_change: bool,
    pub index: u32,
    pub index_change: bool,
    pub rng: Rng,
}
