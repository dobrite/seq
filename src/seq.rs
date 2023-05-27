use heapless::Vec;

use super::{
    output::*,
    tick::{Tick, RESOLUTION},
};

pub struct Seq {
    tick: Tick,
    outputs: Vec<Output, 4>,
    output_states: OutputStates,
}

impl Default for Seq {
    fn default() -> Self {
        Self::new(120, Default::default())
    }
}

impl Seq {
    pub fn new(bpm: u32, configs: Vec<Config, 4>) -> Self {
        Seq::new_with_resolution(RESOLUTION, bpm, configs)
    }

    fn new_with_resolution(resolution: u32, bpm: u32, configs: Vec<Config, 4>) -> Self {
        let tick = Tick::new(bpm as f32);
        let mut output_states = Vec::new();
        for _ in 0..configs.len() {
            output_states.push(Default::default()).ok();
        }
        let outputs = Self::build_outputs(resolution, configs);

        Self {
            tick,
            outputs,
            output_states,
        }
    }

    fn build_outputs(resolution: u32, configs: Vec<Config, 4>) -> Vec<Output, 4> {
        configs
            .iter()
            .map(|config| Output::new(resolution, *config))
            .collect()
    }

    pub fn tick(&mut self) -> &OutputStates {
        for output in self.outputs.iter_mut() {
            output.tick(self.tick.count);
        }

        for (output, state) in self.outputs.iter().zip(self.output_states.iter_mut()) {
            output.state(state);
        }

        self.tick.count += 1;

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

    pub fn set_length(&mut self, index: usize, length: Length) {
        self.outputs[index].set_length(length);
    }

    pub fn set_density(&mut self, index: usize, density: Density) {
        self.outputs[index].set_density(density);
    }

    fn state(&self) -> &OutputStates {
        &self.output_states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_new() {
        let resolution = 1_920;
        let mut configs: Vec<Config, 4> = Vec::new();
        for _ in 0..4 {
            configs.push(Default::default()).unwrap();
        }
        let seq = Seq::new_with_resolution(resolution, 120, configs);
        let result = seq.state();

        let expected = OutputState {
            on: false,
            edge_change: false,
        };

        assert_eq!(4, result.len());
        assert_eq!(expected, result[0]);
        assert_eq!(expected, result[1]);
        assert_eq!(expected, result[2]);
        assert_eq!(expected, result[3]);
    }

    #[test]
    fn it_updates() {
        let resolution = 2;
        let mut configs: Vec<Config, 4> = Vec::new();
        for _ in 0..1 {
            configs.push(Default::default()).unwrap();
        }
        let mut seq = Seq::new_with_resolution(resolution, 120, configs);

        seq.tick();
        let result = seq.state();

        let expected = OutputState {
            on: true,
            edge_change: true,
        };

        assert_eq!(1, result.len());
        assert_eq!(expected, result[0]);

        seq.tick();
        let result = seq.state();

        let expected = OutputState {
            on: false,
            edge_change: true,
        };

        assert_eq!(1, result.len());
        assert_eq!(expected, result[0]);
    }
}
