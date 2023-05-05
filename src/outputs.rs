use crate::{Output, OutputState, Pwm, Rate, Tick};

use heapless::Vec;

pub struct Outputs {
    outputs: Vec<Output, 4>,
    resolution: u32,
    tick: Tick,
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
            outputs,
            resolution,
            tick: Tick { count: 1 },
        }
    }

    pub fn update(&mut self) {
        for o in self.outputs.iter_mut() {
            o.update();
        }

        if self.tick.count == self.resolution {
            self.tick.count = 1;
        } else {
            self.tick.count += 1;
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
            tick: &self.tick,
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
            tick: &Tick { count: 1 },
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn it_updates() {
        let resolution = 24;
        let mut outputs = Outputs::new(1, resolution);
        outputs.update();
        let result = outputs.state();

        let mut expected_states = Vec::new();
        expected_states.push(State::On).unwrap();
        let expected = OutputState {
            outputs: expected_states,
            tick: &Tick { count: 2 },
        };

        assert_eq!(expected, result);
    }
}
