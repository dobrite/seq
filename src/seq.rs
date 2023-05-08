use heapless::Vec;

use crate::lane::Gate;
use crate::{LaneState, LaneStates, Prob, Pwm, Rate};

pub struct Seq {
    count: u32,
    lanes: Vec<Gate, 4>,
}

impl Default for Seq {
    fn default() -> Self {
        Self::new(4, 1_920)
    }
}

impl Seq {
    pub fn new(num: usize, resolution: u32) -> Self {
        let lanes = {
            let mut o = Vec::new();
            for _ in 0..num {
                let lane = Gate::new(resolution, Rate::Unity, Pwm::P50, Prob::P100);
                o.push(lane).ok();
            }
            o
        };

        Self { count: 0, lanes }
    }

    pub fn tick(&mut self) -> LaneStates {
        for o in self.lanes.iter_mut() {
            o.tick(self.count);
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
        self.lanes
            .iter()
            .map(|o| LaneState {
                on: o.on,
                edge_change: o.edge_change,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_new() {
        let resolution = 1_920;
        let seq = Seq::new(4, resolution);
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
        let mut seq = Seq::new(1, resolution);
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
