use super::components::{Prob, Pwm, Rate, Rng};

#[derive(Debug, PartialEq)]
pub struct Gate {
    cycle_target: u32,
    off_target: u32,
    pub(crate) on: bool,
    pub(crate) edge_change: bool,
    pwm: Pwm,
    rate: Rate,
    resolution: u32,
    rng: Rng,
    skip_cycle: bool,
}

impl Default for Gate {
    fn default() -> Self {
        Self::new(1_920, Rate::Unity, Pwm::P50, Prob::P100)
    }
}

impl Gate {
    pub fn new(resolution: u32, rate: Rate, pwm: Pwm, prob: Prob) -> Self {
        let mut gate = Self {
            cycle_target: 0,
            off_target: 0,
            on: true,
            edge_change: false,
            pwm,
            rate,
            resolution,
            rng: Rng::new(prob),
            skip_cycle: false,
        };

        gate.calc_targets();
        gate.calc_skip_cycle();
        gate.calc_initial_state();

        gate
    }

    fn calc_targets(&mut self) {
        self.calc_cycle_target();
        self.calc_off_target();
    }

    fn calc_cycle_target(&mut self) {
        self.cycle_target = match self.rate {
            Rate::Div(div) => div * self.resolution as f32,
            Rate::Unity => self.resolution as f32,
            Rate::Mult(mult) => (1.0 / mult) * self.resolution as f32,
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
        let initial_on = self.on;

        let cycle_mod = count % self.cycle_target;
        if cycle_mod % self.cycle_target == 0 {
            self.calc_skip_cycle();
            self.on = !self.skip_cycle;
            self.edge_change = initial_on != self.on;
            return;
        }

        let off_mod = count % self.off_target;
        if off_mod % self.off_target == 0 {
            self.on = false;
        }

        self.edge_change = initial_on != self.on;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ON: bool = true;
    const OFF: bool = false;

    #[test]
    fn it_new() {
        let rate = Rate::Unity;
        let pwm = Pwm::P50;
        let prob = Prob::P100;
        let gate = Gate::new(1_920, rate, pwm, prob);

        let expected = Gate {
            cycle_target: 1_920,
            off_target: 960,
            on: true,
            edge_change: false,
            pwm: Pwm::P50,
            rate,
            resolution: 1_920,
            rng: Rng::new(prob),
            skip_cycle: false,
        };

        assert_eq!(expected, gate);
    }

    #[test]
    fn it_updates_on_through_two_full_cycles_at_pwm_p50() {
        let mut gate = Gate::new(1_920, Rate::Unity, Pwm::P50, Prob::P100);

        assert_eq!(ON, gate.on);

        gate.tick(0);
        assert_eq!(ON, gate.on);

        gate.tick(480);
        assert_eq!(ON, gate.on);

        gate.tick(960);
        assert_eq!(OFF, gate.on);

        gate.tick(1_440);
        assert_eq!(OFF, gate.on);

        gate.tick(1_920);
        assert_eq!(ON, gate.on);

        gate.tick(2_400);
        assert_eq!(ON, gate.on);

        gate.tick(2_880);
        assert_eq!(OFF, gate.on);

        gate.tick(3_360);
        assert_eq!(OFF, gate.on);

        gate.tick(3_840);
        assert_eq!(ON, gate.on);
    }

    #[test]
    fn it_updates_edge_change_through_two_full_cycles_at_pwm_p50() {
        let mut gate = Gate::new(1_920, Rate::Unity, Pwm::P50, Prob::P100);

        gate.tick(0);
        assert_eq!(OFF, gate.edge_change);
        gate.tick(1);
        assert_eq!(OFF, gate.edge_change);
        gate.tick(2);
        assert_eq!(OFF, gate.edge_change);

        gate.tick(959);
        assert_eq!(OFF, gate.edge_change);
        gate.tick(960);
        assert_eq!(ON, gate.edge_change);
        gate.tick(961);
        assert_eq!(OFF, gate.edge_change);

        gate.tick(1_919);
        assert_eq!(OFF, gate.edge_change);
        gate.tick(1_920);
        assert_eq!(ON, gate.edge_change);
        gate.tick(1_921);
        assert_eq!(OFF, gate.edge_change);

        gate.tick(2_879);
        assert_eq!(OFF, gate.edge_change);
        gate.tick(2_880);
        assert_eq!(ON, gate.edge_change);
        gate.tick(2_881);
        assert_eq!(OFF, gate.edge_change);

        gate.tick(3_839);
        assert_eq!(OFF, gate.edge_change);
        gate.tick(3_840);
        assert_eq!(ON, gate.edge_change);
        gate.tick(3_841);
        assert_eq!(OFF, gate.edge_change);
    }

    #[test]
    fn it_ticks_at_twice_the_rate_with_rate_times_2() {
        let rate = Rate::Mult(2.0);
        let pwm = Pwm::P50;
        let prob = Prob::P100;
        let mut gate = Gate::new(1_920, rate, pwm, prob);

        assert_eq!(960, gate.cycle_target);
        assert_eq!(480, gate.off_target);
        assert_eq!(rate, gate.rate);

        assert_eq!(ON, gate.on);

        gate.tick(480);
        assert_eq!(OFF, gate.on);

        gate.tick(960);
        assert_eq!(ON, gate.on);

        gate.tick(1_440);
        assert_eq!(OFF, gate.on);

        gate.tick(1_920);
        assert_eq!(ON, gate.on);
    }

    #[test]
    fn it_ticks_at_div_five_point_three_the_rate() {
        let rate = Rate::Div(5.333_333_5);
        let pwm = Pwm::P50;
        let prob = Prob::P100;
        let mut gate = Gate::new(1_920, rate, pwm, prob);

        assert_eq!(5_120, gate.off_target);
        assert_eq!(10_240, gate.cycle_target);
        assert_eq!(rate, gate.rate);

        assert_eq!(ON, gate.on);

        gate.tick(5_119);
        assert_eq!(ON, gate.on);

        gate.tick(5_120);
        assert_eq!(OFF, gate.on);

        gate.tick(10_239);
        assert_eq!(OFF, gate.on);

        gate.tick(10_240);
        assert_eq!(ON, gate.on);
    }

    #[test]
    fn it_skips_cycles_based_on_prob() {
        let rate = Rate::Unity;
        let pwm = Pwm::P50;
        let prob = Prob::P10;
        let mut gate = Gate::new(1_920, rate, pwm, prob);

        assert_eq!(OFF, gate.on);

        gate.tick(1_920);
        assert_eq!(OFF, gate.on);

        gate.tick(3_840);
        assert_eq!(OFF, gate.on);

        gate.tick(5_760);
        assert_eq!(OFF, gate.on);

        gate.tick(7_680);
        assert_eq!(OFF, gate.on);
    }

    #[test]
    fn it_works_with_pwm_pew() {
        let rate = Rate::Unity;
        let pwm = Pwm::Pew;
        let prob = Prob::P10;
        let mut gate = Gate::new(1_920, rate, pwm, prob);
        gate.tick(1);
    }
}
