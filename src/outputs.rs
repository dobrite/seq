use crate::{Output, OutputState, Pwm, Rate, Tick};

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
                o.push(Output::new(resolution, Rate::Unity)).ok();
            }
            o
        };

        Self {
            count: 1,
            outputs,
            resolution,
        }
    }

    pub fn tick(&mut self) {
        for o in self.outputs.iter_mut() {
            o.tick();
        }

        if self.count == self.resolution {
            self.count = 1;
        } else {
            self.count += 1;
        }
    }

    pub fn set_pwm(&mut self, index: usize, pwm: Pwm) {
        self.outputs[index].set_pwm(pwm);
    }

    pub fn set_rate(&mut self, index: usize, rate: Rate) {
        self.outputs[index].set_rate(rate);
    }

    pub fn state(&self) -> OutputState {
        let outputs = self.outputs.iter().map(|o| o.state).collect();

        OutputState {
            outputs,
            tick: Tick {
                major: self.count == 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::State;

    #[test]
    fn it_new() {
        let resolution = 24;
        let outputs = Outputs::new(4, resolution);
        let result = outputs.state();

        let mut expected_outputs = Vec::new();
        expected_outputs.push(State::On).unwrap();
        expected_outputs.push(State::On).unwrap();
        expected_outputs.push(State::On).unwrap();
        expected_outputs.push(State::On).unwrap();

        let expected = OutputState {
            outputs: expected_outputs,
            tick: Tick { major: true },
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn it_updates() {
        let resolution = 24;
        let mut outputs = Outputs::new(1, resolution);
        outputs.tick();
        let result = outputs.state();

        let mut expected_states = Vec::new();
        expected_states.push(State::On).unwrap();
        let expected = OutputState {
            outputs: expected_states,
            tick: Tick { major: false },
        };

        assert_eq!(expected, result);
    }
}
