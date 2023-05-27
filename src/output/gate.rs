use super::{
    components::{Prob, Pwm, Rate, Rng},
    Config,
};

#[derive(Debug, PartialEq)]
pub struct Gate {
    config: Config,
    cycle_enabled: bool,
    cycle_target: u32,
    off_target: u32,
    pub(crate) on: bool,
    pub(crate) edge_change: bool,
    resolution: u32,
    rng: Rng,
}

impl Default for Gate {
    fn default() -> Self {
        Self::new(1_920, Default::default())
    }
}

impl Gate {
    pub fn new(resolution: u32, config: Config) -> Self {
        let mut gate = Self {
            config,
            cycle_enabled: true,
            cycle_target: 0,
            off_target: 0,
            on: false,
            edge_change: false,
            resolution,
            rng: Rng::new(config.prob),
        };

        gate.calc_targets();
        gate.calc_cycle_enabled();

        gate
    }

    fn calc_targets(&mut self) {
        self.calc_cycle_target();
        self.calc_off_target();
    }

    fn calc_cycle_target(&mut self) {
        self.cycle_target = (Into::<f32>::into(self.config.rate) * self.resolution as f32) as u32
    }

    fn calc_off_target(&mut self) {
        let ratio: f32 = self.config.pwm.into();
        self.off_target = (ratio * self.cycle_target as f32) as u32
    }

    fn calc_cycle_enabled(&mut self) {
        self.cycle_enabled = self.rng.rand_bool();
    }

    pub fn set_prob(&mut self, prob: Prob) {
        self.rng = Rng::new(prob);
        self.calc_cycle_enabled();
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        self.config.pwm = pwm;
        self.calc_targets();
    }

    pub fn set_rate(&mut self, rate: Rate) {
        self.config.rate = rate;
        self.calc_targets();
    }

    pub fn tick(&mut self, count: u32) {
        let initial_on = self.on;

        if self.turn_on(count) {
            self.calc_cycle_enabled();
            self.on = self.cycle_enabled;
        } else if self.turn_off(count) {
            self.on = false;
        }

        self.edge_change = initial_on != self.on;
    }

    #[inline(always)]
    fn turn_on(&self, count: u32) -> bool {
        count % self.cycle_target == 0
    }

    #[inline(always)]
    fn turn_off(&self, count: u32) -> bool {
        count % self.off_target == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{components::Frac, Density, Length, OutputType},
        *,
    };

    const ON: bool = true;
    const OFF: bool = false;

    #[test]
    fn it_new() {
        let rate = Rate::Unity;
        let pwm = Pwm::P50;
        let prob = Prob::P100;
        let length = Length(16);
        let density = Density(4);
        let output_type = OutputType::Gate;
        let config = Config {
            density,
            length,
            output_type,
            prob,
            pwm,
            rate,
        };

        let gate = Gate::new(1_920, config);

        let expected = Gate {
            config,
            cycle_enabled: true,
            cycle_target: 1_920,
            off_target: 960,
            on: false,
            edge_change: false,
            resolution: 1_920,
            rng: Rng::new(prob),
        };

        assert_eq!(expected, gate);
    }

    #[test]
    fn it_updates_on_through_two_full_cycles_at_pwm_p50() {
        let mut gate = Gate::new(1_920, Default::default());

        assert_eq!(OFF, gate.on);

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
        let mut gate = Gate::new(1_920, Default::default());

        assert_eq!(OFF, gate.edge_change);

        gate.tick(0);
        assert_eq!(ON, gate.edge_change);
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
    fn it_ticks_at_mult_two_point_zero_times_the_rate() {
        let rate = Rate::Mult(2, Frac::Zero);
        let config = Config {
            rate,
            ..Default::default()
        };
        let mut gate = Gate::new(1_920, config);

        assert_eq!(960, gate.cycle_target);
        assert_eq!(480, gate.off_target);
        assert_eq!(rate, gate.config.rate);

        assert_eq!(OFF, gate.on);

        gate.tick(0);
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
    fn it_ticks_at_div_five_point_one_third_the_rate() {
        let rate = Rate::Div(5, Frac::OneThird);
        let config = Config {
            rate,
            ..Default::default()
        };
        let mut gate = Gate::new(1_920, config);

        assert_eq!(5_120, gate.off_target);
        assert_eq!(10_240, gate.cycle_target);
        assert_eq!(rate, gate.config.rate);

        assert_eq!(OFF, gate.on);

        gate.tick(0);
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
        let prob = Prob::P10;
        let config = Config {
            prob,
            ..Default::default()
        };
        let mut gate = Gate::new(1_920, config);

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
        let pwm = Pwm::Pew;
        let config = Config {
            pwm,
            ..Default::default()
        };
        let mut gate = Gate::new(1_920, config);
        gate.tick(1);
    }
}
