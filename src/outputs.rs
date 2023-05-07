use heapless::Vec;

use crate::{output::Output, OutputState, OutputStates, Prob, Pwm, Rate};

pub struct Outputs {
    count: u32,
    outputs: Vec<Output, 4>,
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
                o.push(Output::new(resolution, Prob::P100, Rate::Unity))
                    .ok();
            }
            o
        };

        Self { count: 0, outputs }
    }

    pub fn tick(&mut self) -> OutputStates {
        for o in self.outputs.iter_mut() {
            o.tick(self.count);
        }

        self.count += 1;

        self.state()
    }

    pub fn set_prob(&mut self, index: usize, prob: Prob) {
        self.outputs[index].set_prob(prob);
    }

    pub fn set_pwm(&mut self, index: usize, pwm: Pwm) {
        self.outputs[index].set_pwm(pwm);
    }

    pub fn set_rate(&mut self, index: usize, rate: Rate) {
        self.outputs[index].set_rate(rate);
    }

    fn state(&self) -> OutputStates {
        let outputs = self
            .outputs
            .iter()
            .map(|o| OutputState {
                on: o.on,
                edge_change: o.edge_change,
            })
            .collect();

        OutputStates { outputs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_new() {
        let resolution = 1_920;
        let outputs = Outputs::new(4, resolution);
        let result = outputs.state();

        let mut expected_outputs = Vec::new();
        expected_outputs
            .push(OutputState {
                on: true,
                edge_change: false,
            })
            .unwrap();
        expected_outputs
            .push(OutputState {
                on: true,
                edge_change: false,
            })
            .unwrap();
        expected_outputs
            .push(OutputState {
                on: true,
                edge_change: false,
            })
            .unwrap();
        expected_outputs
            .push(OutputState {
                on: true,
                edge_change: false,
            })
            .unwrap();

        let expected = OutputStates {
            outputs: expected_outputs,
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn it_updates() {
        let resolution = 2;
        let mut outputs = Outputs::new(1, resolution);
        outputs.tick();
        let result = outputs.state();

        let mut expected_states = Vec::new();
        expected_states
            .push(OutputState {
                on: true,
                edge_change: false,
            })
            .unwrap();
        let expected = OutputStates {
            outputs: expected_states,
        };

        assert_eq!(expected, result);

        outputs.tick();
        let result = outputs.state();

        let mut expected_states = Vec::new();
        expected_states
            .push(OutputState {
                on: false,
                edge_change: true,
            })
            .unwrap();
        let expected = OutputStates {
            outputs: expected_states,
        };

        assert_eq!(expected, result);
    }
}
