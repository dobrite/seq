use heapless::Vec;

use super::{
    output::*,
    tick::{Tick, RESOLUTION},
};

pub struct Seq {
    tick: Tick,
    outputs: Vec<Output, 4>,
    output_states: OutputStates,
    resolution: u32,
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
        let tick = Tick::new(bpm);
        let mut output_states = Vec::new();
        output_states.resize_default(configs.len()).ok();
        let outputs = configs
            .iter()
            .map(|config| Output::new(resolution, &tick, config.clone()))
            .collect();

        Self {
            tick,
            outputs,
            output_states,
            resolution,
        }
    }

    pub fn tick_duration_micros(&self) -> u64 {
        self.tick.duration_micros
    }

    pub fn tick(&mut self) {
        for (output, state) in self.outputs.iter_mut().zip(self.output_states.iter_mut()) {
            output.tick(self.tick.count, state);
        }

        self.tick.count += 1;
    }

    pub fn get_index(&self, index: usize) -> usize {
        self.output_states[index].index
    }

    pub fn get_index_change(&self, index: usize) -> bool {
        self.output_states[index].index_change
    }

    pub fn get_on(&self, index: usize) -> bool {
        self.output_states[index].on
    }

    pub fn get_on_change(&self, index: usize) -> bool {
        self.output_states[index].on_change
    }

    pub fn resolution(&self) -> u32 {
        self.resolution
    }

    pub fn set_bpm(&mut self, bpm: u32) {
        self.tick.set_bpm(bpm);
    }

    pub fn set_prob(&mut self, index: usize, prob: Prob) {
        self.outputs[index].set_prob(prob);
    }

    pub fn set_pwm(&mut self, index: usize, pwm: Pwm) {
        self.outputs[index].set_pwm(&self.tick, pwm);
    }

    pub fn set_rate(&mut self, index: usize, rate: Rate) {
        self.outputs[index].set_rate(&self.tick, rate);
    }

    pub fn set_sequence(&mut self, index: usize, length: Length, density: Density) {
        self.outputs[index].set_sequence(length, density);
    }

    pub fn set_output_type(&mut self, index: usize, output_type: OutputType) {
        self.outputs[index].set_output_type(&self.tick, output_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_new() {
        let resolution = 1_920;
        let mut configs: Vec<Config, 4> = Vec::new();
        configs.resize_default(4).ok();
        let seq = Seq::new_with_resolution(resolution, 120, configs);

        let expected = OutputState {
            index: 0,
            index_change: false,
            on: false,
            on_change: false,
            rng: Rng::new(),
        };

        assert_eq!(4, seq.output_states.len());
        assert_eq!(expected, seq.output_states[0]);
        assert_eq!(expected, seq.output_states[1]);
        assert_eq!(expected, seq.output_states[2]);
        assert_eq!(expected, seq.output_states[3]);
    }

    #[test]
    fn it_updates() {
        let resolution = 2;
        let mut configs: Vec<Config, 4> = Vec::new();
        configs.resize_default(1).ok();
        let mut seq = Seq::new_with_resolution(resolution, 120, configs);

        seq.tick();

        let expected = OutputState {
            index: 0,
            index_change: false,
            on: true,
            on_change: true,
            rng: Rng::new(),
        };

        assert_eq!(1, seq.output_states.len());
        assert_eq!(expected, seq.output_states[0]);

        seq.tick();

        let expected = OutputState {
            index: 0,
            index_change: false,
            on: false,
            on_change: true,
            rng: Rng::new(),
        };

        assert_eq!(1, seq.output_states.len());
        assert_eq!(expected, seq.output_states[0]);
    }
}
