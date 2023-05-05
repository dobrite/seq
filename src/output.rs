use crate::{Pwm, Rand, Rate};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq)]
pub struct Output {
    count: u32,
    cycle_target: u32,
    off_target: u32,
    pwm: Pwm,
    rand: Rand,
    rate: Rate,
    resolution: u32,
    pub state: State,
}

impl Default for Output {
    fn default() -> Self {
        Self::new(1_920, Rand::R100, Rate::Unity)
    }
}

impl Output {
    pub fn new(resolution: u32, rand: Rand, rate: Rate) -> Self {
        let pwm = Pwm::P50;

        let mut output = Self {
            count: 1,
            cycle_target: 0,
            off_target: 0,
            pwm,
            rand,
            rate,
            resolution,
            state: State::On,
        };

        output.calc();

        output
    }

    fn calc_cycle_target(&mut self) {
        self.cycle_target = match self.rate {
            Rate::Div(div) => div as f32 * self.resolution as f32,
            Rate::Unity => self.resolution as f32,
            Rate::Mult(mult) => (1.0 / mult as f32) * self.resolution as f32,
        } as u32
    }

    fn calc(&mut self) {
        self.calc_cycle_target();
        self.calc_off_target();
    }

    fn calc_off_target(&mut self) {
        let ratio: f32 = self.pwm.into();
        self.off_target = (ratio * self.cycle_target as f32) as u32
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        self.pwm = pwm;
        self.calc();
    }

    pub fn set_rate(&mut self, rate: Rate) {
        self.rate = rate;
        self.calc();
    }

    pub fn tick(&mut self) {
        if self.count == self.cycle_target {
            self.count = 1;
        } else {
            self.count += 1;
        }

        if self.count <= self.off_target {
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
        let rand = Rand::R100;
        let rate = Rate::Unity;
        let output = Output::new(1_920, rand, rate);

        let expected = Output {
            count: 1,
            cycle_target: 1_920,
            off_target: 960,
            pwm: Pwm::P50,
            rand,
            rate,
            resolution: 1_920,
            state: State::On,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates() {
        let rand = Rand::R100;
        let rate = Rate::Unity;
        let mut output = Output::new(1_920, rand, rate);
        output.tick();

        let expected = Output {
            cycle_target: 1_920,
            count: 2,
            off_target: 960,
            pwm: Pwm::P50,
            rand,
            rate,
            resolution: 1_920,
            state: State::On,
        };

        assert_eq!(expected, output);
    }

    #[test]

    fn it_updates_through_a_full_cycle() {
        let rand = Rand::R100;
        let rate = Rate::Unity;
        let mut output = Output::new(4, rand, rate);

        let mut expected = Output {
            count: 1,
            cycle_target: 4,
            off_target: 2,
            pwm: Pwm::P50,
            rand,
            rate,
            resolution: 4,
            state: State::On,
        };

        assert_eq!(expected, output);

        output.tick();

        expected = Output {
            count: 2,
            cycle_target: 4,
            off_target: 2,
            pwm: Pwm::P50,
            rand,
            rate,
            resolution: 4,
            state: State::On,
        };

        assert_eq!(expected, output);

        output.tick();

        expected = Output {
            count: 3,
            cycle_target: 4,
            off_target: 2,
            pwm: Pwm::P50,
            rand,
            rate,
            resolution: 4,
            state: State::Off,
        };

        assert_eq!(expected, output);

        output.tick();

        expected = Output {
            count: 4,
            cycle_target: 4,
            off_target: 2,
            pwm: Pwm::P50,
            rand,
            rate,
            resolution: 4,
            state: State::Off,
        };

        assert_eq!(expected, output);

        output.tick();

        expected = Output {
            count: 1,
            cycle_target: 4,
            off_target: 2,
            pwm: Pwm::P50,
            rand: Rand::R100,
            rate: Rate::Unity,
            resolution: 4,
            state: State::On,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_ticks_at_twice_the_rate_with_rate_times_2() {
        let rand = Rand::R100;
        let rate = Rate::Mult(2);
        let mut output = Output::new(4, rand, rate);
        output.tick();

        let expected = Output {
            count: 2,
            cycle_target: 2,
            off_target: 1,
            pwm: Pwm::P50,
            rand,
            rate,
            resolution: 4,
            state: State::Off,
        };

        assert_eq!(expected, output);
    }
}
