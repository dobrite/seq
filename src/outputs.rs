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
                o.push(Output::new(resolution)).ok();
            }
            o
        };

        Self {
            count: 1,
            outputs,
            resolution,
        }
    }

    pub fn update(&mut self) {
        for o in self.outputs.iter_mut() {
            o.update();
        }

        if self.count == self.resolution {
            self.count = 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::State;

    #[test]
    fn it_new() {
        let outputs = Outputs::new(4, 24);
        let result = outputs.state();

        let mut expected_outputs = Vec::new();
        expected_outputs.push(State::On).unwrap();
        expected_outputs.push(State::On).unwrap();
        expected_outputs.push(State::On).unwrap();
        expected_outputs.push(State::On).unwrap();

        let expected = OutputState {
            outputs: expected_outputs,
            count: 1,
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn it_updates() {
        let mut outputs = Outputs::new(1, 24);
        outputs.update();
        let result = outputs.state();

        let mut expected_states = Vec::new();
        expected_states.push(State::On).unwrap();
        let expected = OutputState {
            outputs: expected_states,
            count: 2,
        };

        assert_eq!(expected, result);
    }
}
