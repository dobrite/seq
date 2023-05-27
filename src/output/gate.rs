use heapless::Vec;

use super::{
    components::{euclid, Density, Length, OutputType, Prob, Pwm, Rate, Rng},
    Config,
};

#[derive(Debug, PartialEq)]
pub struct Gate {
    config: Config,
    cycle_target: u32,
    off_target: u32,
    pub(crate) on: bool,
    pub(crate) edge_change: bool,
    resolution: u32,
    rng: Rng,
    sequence: Vec<bool, 16>,
}

impl Default for Gate {
    fn default() -> Self {
        Self::new(1_920, Default::default())
    }
}

impl Gate {
    pub fn new(resolution: u32, config: Config) -> Self {
        let mut sequence: Vec<bool, 16> = Vec::new();
        for _ in 0..16 {
            sequence.push(false).unwrap();
        }
        euclid(config.density, config.length, &mut sequence);

        let mut gate = Self {
            config,
            cycle_target: 0,
            off_target: 0,
            on: false,
            edge_change: false,
            resolution,
            rng: Rng::new(config.prob),
            sequence,
        };

        gate.calc_targets();

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

    pub fn set_prob(&mut self, prob: Prob) {
        self.rng = Rng::new(prob);
    }

    pub fn set_pwm(&mut self, pwm: Pwm) {
        self.config.pwm = pwm;
        self.calc_targets();
    }

    pub fn set_rate(&mut self, rate: Rate) {
        self.config.rate = rate;
        self.calc_targets();
    }

    pub fn set_length(&mut self, length: Length) {
        self.config.length = length;
        self.sequence.truncate(length.0 as usize);
        euclid(self.config.density, self.config.length, &mut self.sequence);
    }

    pub fn set_density(&mut self, density: Density) {
        self.config.density = density;
        euclid(self.config.density, self.config.length, &mut self.sequence);
    }

    pub fn tick(&mut self, count: u32) {
        let initial_on = self.on;

        if self.is_cycle_starting(count) {
            self.on = self.is_on(count);
        } else if self.is_cycle_finished(count) {
            self.on = false;
        }

        self.edge_change = initial_on != self.on;
    }

    #[inline(always)]
    fn is_cycle_starting(&self, count: u32) -> bool {
        count % self.cycle_target == 0
    }

    #[inline(always)]
    fn is_on(&mut self, count: u32) -> bool {
        match self.config.output_type {
            OutputType::Gate => self.rng.rand_bool(),
            OutputType::Euclid => {
                let index = count / self.cycle_target % self.config.length.0;
                self.sequence[index as usize]
            }
        }
    }

    #[inline(always)]
    fn is_cycle_finished(&self, count: u32) -> bool {
        match self.config.output_type {
            OutputType::Gate => count % self.off_target == 0,
            OutputType::Euclid => true,
        }
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
        let mut sequence: Vec<bool, 16> = Vec::new();
        for _ in 0..16 {
            sequence.push(false).unwrap();
        }
        euclid(config.density, config.length, &mut sequence);

        let gate = Gate::new(1_920, config);

        let expected = Gate {
            config,
            cycle_target: 1_920,
            off_target: 960,
            on: false,
            edge_change: false,
            resolution: 1_920,
            rng: Rng::new(prob),
            sequence,
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

    #[test]
    fn it_updates_on_at_length_sixteen_at_density_four() {
        let mut euclid = Gate::new(
            1_920,
            Config {
                output_type: OutputType::Euclid,
                ..Default::default()
            },
        );

        euclid.tick(0);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 2);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 3);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 4);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920 * 5);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 6);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 7);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 8);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920 * 9);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 10);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 11);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 12);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920 * 13);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 14);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 15);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 16);
        assert_eq!(ON, euclid.on);
    }

    #[test]
    fn it_updates_edge_change_at_length_sixteen_at_density_four() {
        let mut euclid = Gate::new(
            1_920,
            Config {
                output_type: OutputType::Euclid,
                ..Default::default()
            },
        );

        assert_eq!(OFF, euclid.edge_change);

        euclid.tick(0);
        assert_eq!(ON, euclid.edge_change);
        euclid.tick(1);
        assert_eq!(ON, euclid.edge_change);
        euclid.tick(2);
        assert_eq!(OFF, euclid.edge_change);

        euclid.tick(1_919);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(1_920);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(1_921);
        assert_eq!(OFF, euclid.edge_change);

        euclid.tick(3_839);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(3_840);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(3_841);
        assert_eq!(OFF, euclid.edge_change);

        euclid.tick(5_759);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(5_760);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(5_761);
        assert_eq!(OFF, euclid.edge_change);

        euclid.tick(7_679);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(7_680);
        assert_eq!(ON, euclid.edge_change);
        euclid.tick(7_681);
        assert_eq!(ON, euclid.edge_change);
    }
}
