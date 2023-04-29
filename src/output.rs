use crate::Pwm;
use crate::Rate;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
pub struct Output {
    count: u32,
    pub state: State,
    pwm: Pwm,
    rate: Rate,
    resolution: u32,
}

impl Default for Output {
    fn default() -> Self {
        Self::new(1_920)
    }
}

impl Output {
    pub fn new(resolution: u32) -> Self {
        Self {
            count: 0,
            pwm: Pwm::P(50),
            rate: Rate::Unity,
            resolution,
            state: State::On,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_new() {
        let output = Output::new(1_920);

        let expected = Output {
            count: 0,
            pwm: Pwm::P(50),
            rate: Rate::Unity,
            resolution: 1_920,
            state: State::On,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates() {
        let mut output = Output::new(1_920);
        output.update();

        let expected = Output {
            count: 1,
            pwm: Pwm::P(50),
            rate: Rate::Unity,
            resolution: 1_920,
            state: State::On,
        };

        assert_eq!(expected, output);
    }
}
