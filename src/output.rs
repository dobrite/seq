use crate::{rng::Rng, Prob, Pwm, Rate};

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
    rate: Rate,
    resolution: u32,
    rng: Rng,
    skip_cycle: bool,
    pub state: State,
}

impl Default for Output {
    fn default() -> Self {
        Self::new(1_920, Prob::P100, Rate::Unity)
    }
}

impl Output {
    pub fn new(resolution: u32, prob: Prob, rate: Rate) -> Self {
        let pwm = Pwm::P50;

        let mut output = Self {
            count: 1,
            cycle_target: 0,
            off_target: 0,
            pwm,
            rate,
            resolution,
            rng: Rng::new(prob),
            skip_cycle: false,
            state: State::On,
        };

        output.calc_targets();
        output.calc_skip_cycle();
        output.calc_initial_state();

        output
    }

    fn calc_targets(&mut self) {
        self.calc_cycle_target();
        self.calc_off_target();
    }

    fn calc_cycle_target(&mut self) {
        self.cycle_target = match self.rate {
            Rate::Div(div) => div as f32 * self.resolution as f32,
            Rate::Unity => self.resolution as f32,
            Rate::Mult(mult) => (1.0 / mult as f32) * self.resolution as f32,
        } as u32
    }

    fn calc_off_target(&mut self) {
        let ratio: f32 = self.pwm.into();
        self.off_target = (ratio * self.cycle_target as f32) as u32
    }

    fn calc_skip_cycle(&mut self) {
        self.skip_cycle = !self.rng.rand_bool();
    }

    fn calc_initial_state(&mut self) {
        self.state = if self.skip_cycle {
            State::Off
        } else {
            State::On
        };
    }

    pub fn set_prob(&mut self, prob: Prob) {
        self.rng = Rng::new(prob);
        self.calc_skip_cycle();
        self.calc_initial_state();
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        self.pwm = pwm;
        self.calc_targets();
    }

    pub fn set_rate(&mut self, rate: Rate) {
        self.rate = rate;
        self.calc_targets();
    }

    pub fn tick(&mut self) {
        if self.count == self.cycle_target {
            self.count = 1;
            self.calc_skip_cycle();
        } else {
            self.count += 1;
        }

        if self.skip_cycle {
            self.state = State::Off;
            return;
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
        let prob = Prob::P100;
        let rate = Rate::Unity;
        let output = Output::new(1_920, prob, rate);

        let expected = Output {
            count: 1,
            cycle_target: 1_920,
            off_target: 960,
            pwm: Pwm::P50,
            rate,
            resolution: 1_920,
            rng: Rng::new(prob),
            skip_cycle: false,
            state: State::On,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates() {
        let mut output = Output::new(1_920, Prob::P100, Rate::Unity);
        output.tick();

        let expected_count = 2;
        let expected_state = State::On;

        assert_eq!(expected_count, output.count);
        assert_eq!(expected_state, output.state);
    }

    #[test]

    fn it_updates_through_a_full_cycle() {
        let mut output = Output::new(4, Prob::P100, Rate::Unity);

        assert_eq!(1, output.count);
        assert_eq!(State::On, output.state);

        output.tick();

        assert_eq!(2, output.count);
        assert_eq!(State::On, output.state);

        output.tick();

        assert_eq!(3, output.count);
        assert_eq!(State::Off, output.state);

        output.tick();

        assert_eq!(4, output.count);
        assert_eq!(State::Off, output.state);

        output.tick();

        assert_eq!(1, output.count);
        assert_eq!(State::On, output.state);
    }

    #[test]
    fn it_ticks_at_twice_the_rate_with_rate_times_2() {
        let prob = Prob::P100;
        let rate = Rate::Mult(2);
        let mut output = Output::new(4, prob, rate);

        assert_eq!(1, output.count);
        assert_eq!(2, output.cycle_target);
        assert_eq!(1, output.off_target);
        assert_eq!(rate, output.rate);
        assert_eq!(State::On, output.state);

        output.tick();

        assert_eq!(2, output.count);
        assert_eq!(State::Off, output.state);
    }

    #[test]
    fn it_skips_cycles_based_on_prob() {
        let prob = Prob::P10;
        let rate = Rate::Unity;
        let mut output = Output::new(4, prob, rate);

        assert_eq!(State::Off, output.state);
        output.tick();
        assert_eq!(State::Off, output.state);
        output.tick();
        assert_eq!(State::Off, output.state);
        output.tick();
        assert_eq!(State::Off, output.state);
        output.tick();
    }
}
