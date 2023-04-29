#![cfg_attr(not(test), no_std)]

use heapless::Vec;

mod pwm;
mod rate;

pub use pwm::Pwm;
pub use rate::Rate;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    On,
    Off,
}

#[derive(Debug)]
struct Output {
    state: State,
    count: u32,
}

impl Output {
    fn new() -> Self {
        Self {
            state: State::On,
            count: 0,
        }
    }

    fn update(&mut self) {
        self.count += 1;
    }
}

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

#[derive(Debug, PartialEq)]
pub struct OutputState {
    outputs: Vec<State, 4>,
    count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

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
            count: 0,
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn it_updates() {
        let mut outputs = Outputs::new(1, 24);
        outputs.update();
        let result = outputs.state();

        let mut expected_outputs = Vec::new();
        expected_outputs.push(State::On).unwrap();
        let expected = OutputState {
            outputs: expected_outputs,
            count: 1,
        };

        assert_eq!(expected, result);
    }
}
