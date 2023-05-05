use crate::{Pwm, Rate};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
pub struct Output {
    count: u32,
    pwm: Pwm,
    rate: Rate,
    resolution: u32,
    pub state: State,
    target: u32,
}

impl Default for Output {
    fn default() -> Self {
        Self::new(1_920, Rate::Unity)
    }
}

impl Output {
    pub fn new(resolution: u32, rate: Rate) -> Self {
        let pwm = Pwm::P50;

        let mut output = Self {
            count: 1,
            pwm,
            rate,
            resolution,
            state: State::On,
            target: 0,
        };

        output.calc_target();

        output
    }

    pub fn calc_target(&mut self) {
        let ratio: f32 = self.pwm.into();
        self.target = (ratio * self.resolution as f32) as u32
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        self.pwm = pwm;
        self.calc_target();
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
        let output = Output::new(1_920, Rate::Unity);

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
        let mut output = Output::new(1_920, Rate::Unity);
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
        let mut output = Output::new(4, Rate::Unity);

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
