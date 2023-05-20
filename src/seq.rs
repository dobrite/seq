use heapless::Vec;

use crate::{
    lane::{Euclid, Gate, Lane, State},
    ticks, LaneStates, Prob, Pwm, Rate,
};

pub struct Seq {
    count: u32,
    lanes: Vec<Lane, 4>,
}

impl Default for Seq {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Seq {
    pub fn new(states: Vec<State, 4>) -> Self {
        let resolution = ticks::resolution();
        let mut lanes = Vec::new();
        Self::build_lanes(resolution, states, &mut lanes);

        Self { count: 0, lanes }
    }

    #[cfg(test)]
    fn new_with_resolution(resolution: u32, states: Vec<State, 4>) -> Self {
        let mut lanes = Vec::new();
        Self::build_lanes(resolution, states, &mut lanes);

        Self { count: 0, lanes }
    }

    fn build_lanes(resolution: u32, states: Vec<State, 4>, lanes: &mut Vec<Lane, 4>) {
        for idx in 0..states.len() {
            let lane = if idx == 0 {
                Lane::Euclid(Euclid::new(resolution, Rate::Unity, 4, 16))
            } else {
                Lane::Gate(Gate::new(resolution, states[idx]))
            };
            lanes.push(lane).ok();
        }
    }

    pub fn tick(&mut self) -> LaneStates {
        for lane in self.lanes.iter_mut() {
            lane.tick(self.count);
        }

        self.count += 1;

        self.state()
    }

    pub fn set_prob(&mut self, index: usize, prob: Prob) {
        self.lanes[index].set_prob(prob);
    }

    pub fn set_pwm(&mut self, index: usize, pwm: Pwm) {
        self.lanes[index].set_pwm(pwm);
    }

    pub fn set_rate(&mut self, index: usize, rate: Rate) {
        self.lanes[index].set_rate(rate);
    }

    fn state(&self) -> LaneStates {
        self.lanes.iter().map(|lane| lane.into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LaneState;

    #[test]
    fn it_new() {
        let resolution = 1_920;
        let mut states: Vec<State, 4> = Vec::new();
        for _ in 0..4 {
            states.push(Default::default()).unwrap();
        }
        let seq = Seq::new_with_resolution(resolution, states);
        let result = seq.state();

        let expected = LaneState {
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
        let mut states: Vec<State, 4> = Vec::new();
        for _ in 0..1 {
            states.push(Default::default()).unwrap();
        }
        let mut seq = Seq::new_with_resolution(resolution, states);
        seq.tick();
        let result = seq.state();

        let expected = LaneState {
            on: true,
            edge_change: false,
        };

        assert_eq!(1, result.len());
        assert_eq!(expected, result[0]);

        seq.tick();
        let result = seq.state();

        let expected = LaneState {
            on: false,
            edge_change: true,
        };

        assert_eq!(1, result.len());
        assert_eq!(expected, result[0]);
    }
}
