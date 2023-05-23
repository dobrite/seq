use heapless::Vec;

use crate::{
    output::{Config, Density, Euclid, Gate, Length, Output, OutputType},
    ticks, OutputStates, Prob, Pwm, Rate,
};

pub struct Seq {
    count: u32,
    outputs: Vec<Output, 4>,
}

impl Default for Seq {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Seq {
    pub fn new(configs: Vec<Config, 4>) -> Self {
        let resolution = ticks::resolution();
        let mut outputs = Vec::new();
        Self::build_outputs(resolution, configs, &mut outputs);

        Self { count: 0, outputs }
    }

    #[cfg(test)]
    fn new_with_resolution(resolution: u32, configs: Vec<Config, 4>) -> Self {
        let mut outputs = Vec::new();
        Self::build_outputs(resolution, configs, &mut outputs);

        Self { count: 0, outputs }
    }

    fn build_outputs(resolution: u32, configs: Vec<Config, 4>, outputs: &mut Vec<Output, 4>) {
        for idx in 0..configs.len() {
            let output = match configs[idx].output_type {
                OutputType::Gate => Output::Gate(Gate::new(resolution, configs[idx])),
                OutputType::Euclid => Output::Euclid(Euclid::new(resolution, configs[idx])),
            };
            outputs.push(output).ok();
        }
    }

    pub fn tick(&mut self) -> OutputStates {
        for output in self.outputs.iter_mut() {
            output.tick(self.count);
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

    pub fn set_length(&mut self, index: usize, length: Length) {
        self.outputs[index].set_length(length);
    }

    pub fn set_density(&mut self, index: usize, density: Density) {
        self.outputs[index].set_density(density);
    }

    fn state(&self) -> OutputStates {
        self.outputs.iter().map(|output| output.into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OutputState;

    #[test]
    fn it_new() {
        let resolution = 1_920;
        let mut configs: Vec<Config, 4> = Vec::new();
        for _ in 0..4 {
            configs.push(Default::default()).unwrap();
        }
        let seq = Seq::new_with_resolution(resolution, configs);
        let result = seq.state();

        let expected = OutputState {
            on: true,
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
        let mut seq = Seq::new_with_resolution(resolution, configs);
        seq.tick();
        let result = seq.state();

        let expected = OutputState {
            on: true,
            edge_change: false,
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
