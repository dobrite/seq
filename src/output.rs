pub use self::{
    components::{euclid, Density, Frac, Length, OutputType, Prob, Pwm, Rate, Rng},
    config::Config,
    output_state::{OutputState, OutputStates},
};
use super::tick::{Tick, RESOLUTION};

mod components;
mod config;
mod output_state;

use heapless::Vec;

#[derive(Debug, PartialEq)]
pub struct Output {
    config: Config,
    cycle_target: u32,
    off_target: u32,
    pub(crate) on: bool,
    pub(crate) edge_change: bool,
    pub(crate) index: u32,
    pub(crate) index_change: bool,
    resolution: u32,
    rng: Rng,
    sequence: Vec<bool, 16>,
}

impl Default for Output {
    fn default() -> Self {
        Self::new(RESOLUTION, &Tick::new(120), Default::default())
    }
}

impl Output {
    pub fn new(resolution: u32, tick: &Tick, config: Config) -> Self {
        let mut sequence: Vec<bool, 16> = Vec::new();
        for _ in 0..config.length.0 {
            sequence.push(false).unwrap();
        }

        let mut output = Self {
            config,
            cycle_target: 0,
            off_target: 0,
            on: false,
            edge_change: false,
            index: 0,
            index_change: false,
            resolution,
            rng: Rng::new(config.prob),
            sequence,
        };

        output.set_output_type(tick, config.output_type);

        output
    }

    fn calc_targets(&mut self, tick: &Tick) {
        self.calc_cycle_target();
        self.calc_off_target(tick);
    }

    fn calc_cycle_target(&mut self) {
        self.cycle_target = (Into::<f32>::into(self.config.rate) * self.resolution as f32) as u32
    }

    fn calc_off_target(&mut self, tick: &Tick) {
        self.off_target = match self.config.output_type {
            OutputType::Gate => self.config.pwm.off_target(tick, self.cycle_target),
            OutputType::Euclid => Pwm::Pew.off_target(tick, self.cycle_target),
        }
    }

    pub fn set_prob(&mut self, prob: Prob) {
        self.rng = Rng::new(prob);
    }

    pub fn set_pwm(&mut self, tick: &Tick, pwm: Pwm) {
        self.config.pwm = pwm;
        self.calc_targets(tick);
    }

    pub fn set_rate(&mut self, tick: &Tick, rate: Rate) {
        self.config.rate = rate;
        self.calc_targets(tick);
    }

    pub fn set_length(&mut self, length: Length) {
        self.config.length = length;
        self.sequence.resize_default(length.0 as usize).ok();
        euclid(self.config.density, self.config.length, &mut self.sequence);
    }

    pub fn set_density(&mut self, density: Density) {
        self.config.density = density;
        euclid(self.config.density, self.config.length, &mut self.sequence);
    }

    pub fn set_output_type(&mut self, tick: &Tick, output_type: OutputType) {
        self.config.output_type = output_type;
        let density = match output_type {
            OutputType::Gate => Density(self.config.length.0),
            OutputType::Euclid => self.config.density,
        };
        let prob = match output_type {
            OutputType::Gate => self.config.prob,
            OutputType::Euclid => Prob::P100,
        };
        self.set_prob(prob);
        euclid(density, self.config.length, &mut self.sequence);
        self.calc_targets(tick);
    }

    pub fn tick(&mut self, count: u32) {
        let initial_on = self.on;
        let initial_index = self.index;

        if self.is_cycle_starting(count) {
            self.index = self.calc_index(count);
            self.on = self.is_on();
        } else if self.is_cycle_finished(count) {
            self.on = false;
        }

        self.edge_change = initial_on != self.on;
        self.index_change = initial_index != self.index;
    }

    pub fn state(&self, state: &mut OutputState) {
        state.edge_change = self.edge_change;
        state.on = self.on;
    }

    #[inline(always)]
    fn calc_index(&self, count: u32) -> u32 {
        count / self.cycle_target % self.config.length.0
    }

    #[inline(always)]
    fn is_cycle_starting(&self, count: u32) -> bool {
        count % self.cycle_target == 0
    }

    #[inline(always)]
    fn is_on(&mut self) -> bool {
        self.rng.rand_bool() && self.sequence[self.index as usize]
    }

    #[inline(always)]
    fn is_cycle_finished(&self, count: u32) -> bool {
        count % self.cycle_target % self.off_target == 0
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
        let length = Length(16);
        let density = Density(16);
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

        let output = Output::new(1_920, &Tick::new(120), config);

        let expected = Output {
            config,
            cycle_target: 1_920,
            off_target: 960,
            on: false,
            edge_change: false,
            index: 0,
            index_change: false,
            resolution: 1_920,
            rng: Rng::new(prob),
            sequence,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates_on_through_two_full_cycles_at_pwm_p50() {
        let mut output = Output::new(1_920, &Tick::new(120), Default::default());

        assert_eq!(OFF, output.on);

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
    fn it_updates_edge_change_through_two_full_cycles_at_pwm_p50() {
        let mut output = Output::new(1_920, &Tick::new(120), Default::default());

        assert_eq!(OFF, output.edge_change);

        output.tick(0);
        assert_eq!(ON, output.edge_change);
        output.tick(1);
        assert_eq!(OFF, output.edge_change);
        output.tick(2);
        assert_eq!(OFF, output.edge_change);

        output.tick(959);
        assert_eq!(OFF, output.edge_change);
        output.tick(960);
        assert_eq!(ON, output.edge_change);
        output.tick(961);
        assert_eq!(OFF, output.edge_change);

        output.tick(1_919);
        assert_eq!(OFF, output.edge_change);
        output.tick(1_920);
        assert_eq!(ON, output.edge_change);
        output.tick(1_921);
        assert_eq!(OFF, output.edge_change);

        output.tick(2_879);
        assert_eq!(OFF, output.edge_change);
        output.tick(2_880);
        assert_eq!(ON, output.edge_change);
        output.tick(2_881);
        assert_eq!(OFF, output.edge_change);

        output.tick(3_839);
        assert_eq!(OFF, output.edge_change);
        output.tick(3_840);
        assert_eq!(ON, output.edge_change);
        output.tick(3_841);
        assert_eq!(OFF, output.edge_change);
    }

    #[test]
    fn it_ticks_at_mult_two_point_zero_times_the_rate() {
        let rate = Rate::Mult(2, Frac::Zero);
        let config = Config {
            rate,
            ..Default::default()
        };
        let mut output = Output::new(1_920, &Tick::new(120), config);

        assert_eq!(960, output.cycle_target);
        assert_eq!(480, output.off_target);
        assert_eq!(rate, output.config.rate);

        assert_eq!(OFF, output.on);

        output.tick(0);
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
    fn it_ticks_at_div_five_point_one_third_the_rate() {
        let rate = Rate::Div(5, Frac::OneThird);
        let config = Config {
            rate,
            ..Default::default()
        };
        let mut output = Output::new(1_920, &Tick::new(120), config);

        assert_eq!(5_120, output.off_target);
        assert_eq!(10_240, output.cycle_target);
        assert_eq!(rate, output.config.rate);

        assert_eq!(OFF, output.on);

        output.tick(0);
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
        let config = Config {
            prob,
            ..Default::default()
        };
        let mut output = Output::new(1_920, &Tick::new(120), config);

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

    #[test]
    fn it_works_with_pwm_pew() {
        let pwm = Pwm::Pew;
        let config = Config {
            pwm,
            ..Default::default()
        };
        let mut output = Output::new(1_920, &Tick::new(120), config);
        output.tick(1);
    }

    #[test]
    fn it_updates_on_at_length_sixteen_at_density_four() {
        let mut output = Output::new(
            1_920,
            &Tick::new(120),
            Config {
                output_type: OutputType::Euclid,
                ..Default::default()
            },
        );

        output.tick(0);
        assert_eq!(ON, output.on);

        output.tick(1_920);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 2);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 3);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 4);
        assert_eq!(ON, output.on);

        output.tick(1_920 * 5);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 6);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 7);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 8);
        assert_eq!(ON, output.on);

        output.tick(1_920 * 9);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 10);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 11);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 12);
        assert_eq!(ON, output.on);

        output.tick(1_920 * 13);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 14);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 15);
        assert_eq!(OFF, output.on);

        output.tick(1_920 * 16);
        assert_eq!(ON, output.on);
    }

    #[test]
    fn it_updates_edge_change_at_length_sixteen_at_density_four() {
        let mut output = Output::new(
            1_920,
            &Tick::new(120),
            Config {
                output_type: OutputType::Euclid,
                ..Default::default()
            },
        );

        assert_eq!(OFF, output.edge_change);

        output.tick(0);
        assert_eq!(ON, output.edge_change);
        output.tick(1);
        assert_eq!(OFF, output.edge_change);
        output.tick(39);
        assert_eq!(ON, output.edge_change);
        output.tick(40);
        assert_eq!(OFF, output.edge_change);

        output.tick(1_919);
        assert_eq!(OFF, output.edge_change);
        output.tick(1_920);
        assert_eq!(OFF, output.edge_change);
        output.tick(1_959);
        assert_eq!(OFF, output.edge_change);

        output.tick(3_839);
        assert_eq!(OFF, output.edge_change);
        output.tick(3_840);
        assert_eq!(OFF, output.edge_change);
        output.tick(3_879);
        assert_eq!(OFF, output.edge_change);

        output.tick(5_759);
        assert_eq!(OFF, output.edge_change);
        output.tick(5_760);
        assert_eq!(OFF, output.edge_change);
        output.tick(5_799);
        assert_eq!(OFF, output.edge_change);

        output.tick(7_679);
        assert_eq!(OFF, output.edge_change);
        output.tick(7_680);
        assert_eq!(ON, output.edge_change);
        output.tick(7_681);
        assert_eq!(OFF, output.edge_change);
        output.tick(7_719);
        assert_eq!(ON, output.edge_change);
        output.tick(7_720);
        assert_eq!(OFF, output.edge_change);
    }

    #[test]
    fn it_updates_index_at_length_sixteen_at_density_four() {
        let mut output = Output::new(
            1_920,
            &Tick::new(120),
            Config {
                output_type: OutputType::Euclid,
                ..Default::default()
            },
        );

        assert_eq!(0, output.index);

        output.tick(0);
        assert_eq!(0, output.index);
        output.tick(1);
        assert_eq!(0, output.index);
        output.tick(39);
        assert_eq!(0, output.index);
        output.tick(40);
        assert_eq!(0, output.index);

        output.tick(1_919);
        assert_eq!(0, output.index);
        output.tick(1_920);
        assert_eq!(1, output.index);
        output.tick(1_959);
        assert_eq!(1, output.index);

        output.tick(3_839);
        assert_eq!(1, output.index);
        output.tick(3_840);
        assert_eq!(2, output.index);
        output.tick(3_879);
        assert_eq!(2, output.index);

        output.tick(5_759);
        assert_eq!(2, output.index);
        output.tick(5_760);
        assert_eq!(3, output.index);
        output.tick(5_799);
        assert_eq!(3, output.index);

        output.tick(7_679);
        assert_eq!(3, output.index);
        output.tick(7_680);
        assert_eq!(4, output.index);
        output.tick(7_681);
        assert_eq!(4, output.index);
        output.tick(7_719);
        assert_eq!(4, output.index);
        output.tick(7_720);
        assert_eq!(4, output.index);
    }

    #[test]
    fn it_updates_index_change_at_length_sixteen_at_density_four() {
        let mut output = Output::new(
            1_920,
            &Tick::new(120),
            Config {
                output_type: OutputType::Euclid,
                ..Default::default()
            },
        );

        assert_eq!(OFF, output.index_change);

        output.tick(0);
        assert_eq!(OFF, output.index_change);
        output.tick(1);
        assert_eq!(OFF, output.index_change);
        output.tick(39);
        assert_eq!(OFF, output.index_change);
        output.tick(40);
        assert_eq!(OFF, output.index_change);

        output.tick(1_919);
        assert_eq!(OFF, output.index_change);
        output.tick(1_920);
        assert_eq!(ON, output.index_change);
        output.tick(1_959);
        assert_eq!(OFF, output.index_change);

        output.tick(3_839);
        assert_eq!(OFF, output.index_change);
        output.tick(3_840);
        assert_eq!(ON, output.index_change);
        output.tick(3_879);
        assert_eq!(OFF, output.index_change);

        output.tick(5_759);
        assert_eq!(OFF, output.index_change);
        output.tick(5_760);
        assert_eq!(ON, output.index_change);
        output.tick(5_799);
        assert_eq!(OFF, output.index_change);

        output.tick(7_679);
        assert_eq!(OFF, output.index_change);
        output.tick(7_680);
        assert_eq!(ON, output.index_change);
        output.tick(7_681);
        assert_eq!(OFF, output.index_change);
        output.tick(7_719);
        assert_eq!(OFF, output.index_change);
        output.tick(7_720);
        assert_eq!(OFF, output.index_change);
    }
}
