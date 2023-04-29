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
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

impl Output {
    pub fn new() -> Self {
        Self {
            count: 0,
            pwm: Pwm::P(50),
            rate: Rate::Unity,
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
        let output = Output::new();

        let expected = Output {
            count: 0,
            pwm: Pwm::P(50),
            rate: Rate::Unity,
            state: State::On,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates() {
        let mut output = Output::new();
        output.update();

        let expected = Output {
            count: 1,
            pwm: Pwm::P(50),
            rate: Rate::Unity,
            state: State::On,
        };

        assert_eq!(expected, output);
    }
}
