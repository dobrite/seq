use crate::{rng::Rng, Prob, Pwm, Rate};

#[derive(Debug, PartialEq)]
pub struct Output {
    cycle_target: u32,
    off_target: u32,
    pub(crate) on: bool,
    pwm: Pwm,
    rate: Rate,
    resolution: u32,
    rng: Rng,
    skip_cycle: bool,
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
            cycle_target: 0,
            off_target: 0,
            on: true,
            pwm,
            rate,
            resolution,
            rng: Rng::new(prob),
            skip_cycle: false,
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
        self.on = !self.skip_cycle
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

    pub fn tick(&mut self, count: u32) {
        let cycle_mod = count % self.cycle_target;
        if cycle_mod % self.cycle_target == 0 {
            self.calc_skip_cycle();
            self.on = !self.skip_cycle;
            return;
        }

        let off_mod = count % self.off_target;
        if off_mod % self.off_target == 0 {
            self.on = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ON: bool = true;
    const OFF: bool = false;

    #[test]
    fn it_new() {
        let prob = Prob::P100;
        let rate = Rate::Unity;
        let output = Output::new(1_920, prob, rate);

        let expected = Output {
            cycle_target: 1_920,
            off_target: 960,
            on: true,
            pwm: Pwm::P50,
            rate,
            resolution: 1_920,
            rng: Rng::new(prob),
            skip_cycle: false,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates_through_two_full_cycles_at_pwm_p50() {
        let mut output = Output::new(1_920, Prob::P100, Rate::Unity);

        assert_eq!(ON, output.on);

        output.tick(0);
        assert_eq!(ON, output.on);

        output.tick(480);
        assert_eq!(ON, output.on);

        output.tick(960);
        assert_eq!(OFF, output.on);

        output.tick(1_440);
        assert_eq!(OFF, output.on);

        output.tick(1_920);
        assert_eq!(ON, output.on);

        output.tick(2_400);
        assert_eq!(ON, output.on);

        output.tick(2_880);
        assert_eq!(OFF, output.on);

        output.tick(3_360);
        assert_eq!(OFF, output.on);

        output.tick(3_840);
        assert_eq!(ON, output.on);
    }

    #[test]
    fn it_ticks_at_twice_the_rate_with_rate_times_2() {
        let prob = Prob::P100;
        let rate = Rate::Mult(2.0);
        let mut output = Output::new(1_920, prob, rate);

        assert_eq!(960, output.cycle_target);
        assert_eq!(480, output.off_target);
        assert_eq!(rate, output.rate);

        assert_eq!(ON, output.on);

        output.tick(480);
        assert_eq!(OFF, output.on);

        output.tick(960);
        assert_eq!(ON, output.on);

        output.tick(1_440);
        assert_eq!(OFF, output.on);

        output.tick(1_920);
        assert_eq!(ON, output.on);
    }

    #[test]
    fn it_ticks_at_div_five_point_three_the_rate() {
        let prob = Prob::P100;
        let rate = Rate::Div(5.333_333_5);
        let mut output = Output::new(1_920, prob, rate);

        assert_eq!(5_120, output.off_target);
        assert_eq!(10_240, output.cycle_target);
        assert_eq!(rate, output.rate);

        assert_eq!(ON, output.on);

        output.tick(5_119);
        assert_eq!(ON, output.on);

        output.tick(5_120);
        assert_eq!(OFF, output.on);

        output.tick(10_239);
        assert_eq!(OFF, output.on);

        output.tick(10_240);
        assert_eq!(ON, output.on);
    }

    #[test]
    fn it_skips_cycles_based_on_prob() {
        let prob = Prob::P10;
        let rate = Rate::Unity;
        let mut output = Output::new(1_920, prob, rate);

        assert_eq!(OFF, output.on);

        output.tick(1_920);
        assert_eq!(OFF, output.on);

        output.tick(3_840);
        assert_eq!(OFF, output.on);

        output.tick(5_760);
        assert_eq!(OFF, output.on);

        output.tick(7_680);
        assert_eq!(OFF, output.on);
    }
}
