#![cfg_attr(not(test), no_std)]

type OutputStates = (OutputState, OutputState, OutputState, OutputState);

#[derive(Debug, PartialEq)]
pub enum OutputState {
    On,
    Off,
}

pub struct Outputs {}

impl Default for Outputs {
    fn default() -> Self {
        Self::new()
    }
}

impl Outputs {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self) -> OutputStates {
        (
            OutputState::On,
            OutputState::On,
            OutputState::On,
            OutputState::On,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_updates() {
        let mut outputs = Outputs::new();
        let result = outputs.update();

        assert_eq!(
            result,
            (
                OutputState::On,
                OutputState::On,
                OutputState::On,
                OutputState::On
            )
        );
    }
}
