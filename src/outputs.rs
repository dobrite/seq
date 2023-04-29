use crate::Output;
use crate::OutputState;

use heapless::Vec;

pub struct Outputs {
    count: u32,
    outputs: Vec<Output, 4>,
    resolution: u32,
}

impl Default for Outputs {
    fn default() -> Self {
        Self::new(4, 1_920)
    }
}

impl Outputs {
    pub fn new(num: usize, resolution: u32) -> Self {
        let outputs = {
            let mut o = Vec::new();
            for _ in 0..num {
                o.push(Output::new()).ok();
            }
            o
        };

        Self {
            count: 0,
            outputs,
            resolution,
        }
    }

    pub fn update(&mut self) {
        for o in self.outputs.iter_mut() {
            o.update();
        }

        if self.count == self.resolution - 1 {
            self.count = 0;
        } else {
            self.count += 1;
        }
    }

    pub fn state(&self) -> OutputState {
        let outputs = self.outputs.iter().map(|o| o.state).collect();

        OutputState {
            outputs,
            count: self.count,
        }
    }
}
