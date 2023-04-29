#![cfg_attr(not(test), no_std)]

use heapless::Vec;

mod output;
mod outputs;
mod pwm;
mod rate;

pub use output::{Output, State};
pub use outputs::Outputs;
pub use pwm::Pwm;
pub use rate::Rate;

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
