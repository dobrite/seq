use heapless::Vec;

use super::Output;

pub type OutputStates = Vec<OutputState, 4>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutputState {
    pub on: bool,
    pub edge_change: bool,
    pub index: u32,
    pub index_change: bool,
}

impl From<&Output> for OutputState {
    fn from(val: &Output) -> Self {
        OutputState {
            on: val.on,
            edge_change: val.edge_change,
            index: val.index,
            index_change: val.index_change,
        }
    }
}
