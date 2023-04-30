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
    target: u32,
}

impl Default for Output {
    fn default() -> Self {
        Self::new(1_920)
    }
}

impl Output {
    pub fn calc_target(pwm: Pwm, resolution: u32) -> u32 {
        let ratio: f32 = pwm.into();
        (ratio * resolution as f32) as u32
    }

    pub fn new(resolution: u32) -> Self {
        let pwm = Pwm::P50;

        Self {
            count: 1,
            pwm,
            rate: Rate::Unity,
            resolution,
            state: State::On,
            target: Self::calc_target(pwm, resolution),
        }
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        self.pwm = pwm;
        self.target = Self::calc_target(self.pwm, self.resolution);
    }

    pub fn update(&mut self) {
        if self.count == self.resolution {
            self.count = 1;
        } else {
            self.count += 1;
        }

        if self.count <= self.target {
            self.state = State::On
        } else {
            self.state = State::Off
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_new() {
        let output = Output::new(1_920);

        let expected = Output {
            count: 1,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            resolution: 1_920,
            state: State::On,
            target: 960,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates() {
        let mut output = Output::new(1_920);
        output.update();

        let expected = Output {
            count: 2,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            resolution: 1_920,
            state: State::On,
            target: 960,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates_through_a_full_cycle() {
        let mut output = Output::new(4);

        let mut expected = Output {
            count: 1,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            resolution: 4,
            state: State::On,
            target: 2,
        };

        assert_eq!(expected, output);

        output.update();

        expected = Output {
            count: 2,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            resolution: 4,
            state: State::On,
            target: 2,
        };

        assert_eq!(expected, output);

        output.update();

        expected = Output {
            count: 3,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            resolution: 4,
            state: State::Off,
            target: 2,
        };

        assert_eq!(expected, output);

        output.update();

        expected = Output {
            count: 4,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            resolution: 4,
            state: State::Off,
            target: 2,
        };

        assert_eq!(expected, output);

        output.update();

        expected = Output {
            count: 1,
            pwm: Pwm::P50,
            rate: Rate::Unity,
            resolution: 4,
            state: State::On,
            target: 2,
        };

        assert_eq!(expected, output);
    }
}
