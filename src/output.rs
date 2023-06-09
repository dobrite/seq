pub use self::{
    components::{
        euclid, Density, Frac, Length, OutputType, Prob, Pwm, Rate, Rng, Sequence, MAX_STEPS,
    },
    config::Config,
    output_state::{OutputState, OutputStates},
};
use super::tick::{Tick, RESOLUTION};

mod components;
mod config;
mod output_state;

#[derive(Debug, PartialEq)]
pub struct Output {
    config: Config,
    cycle_target: u32,
    index: Option<usize>,
    off_target: u32,
    resolution: u32,
}

impl Default for Output {
    fn default() -> Self {
        Self::new(RESOLUTION, &Tick::new(120), Default::default())
    }
}

impl Output {
    pub fn new(resolution: u32, tick: &Tick, config: Config) -> Self {
        let mut output = Self {
            config,
            cycle_target: 0,
            index: Option::None,
            off_target: 0,
            resolution,
        };

        output.set_output_type(tick, output.config.output_type());

        output
    }

    fn calc_targets(&mut self, tick: &Tick) {
        self.calc_cycle_target();
        self.calc_off_target(tick);
    }

    fn calc_cycle_target(&mut self) {
        self.cycle_target = (Into::<f32>::into(self.config.rate()) * self.resolution as f32) as u32
    }

    fn calc_off_target(&mut self, tick: &Tick) {
        self.off_target = match self.config.output_type() {
            OutputType::Gate => self.config.pwm().off_target(tick, self.cycle_target),
            OutputType::Euclid => Pwm::Pew.off_target(tick, self.cycle_target),
        }
    }

    pub fn set_sequence(&mut self, length: Length, density: Density) {
        self.config.set_sequence(length, density);
    }

    pub fn set_output_type(&mut self, tick: &Tick, output_type: OutputType) {
        self.config.set_output_type(output_type);
        self.calc_targets(tick);
    }

    pub fn set_prob(&mut self, prob: Prob) {
        self.config.set_prob(prob);
    }

    pub fn set_pwm(&mut self, tick: &Tick, pwm: Pwm) {
        self.config.set_pwm(pwm);
        self.calc_targets(tick);
    }

    pub fn set_rate(&mut self, tick: &Tick, rate: Rate) {
        self.config.set_rate(rate);
        self.calc_targets(tick);
    }

    pub fn tick(&mut self, count: u32, state: &mut OutputState) {
        let initial_on = state.on;
        let initial_index = state.index;

        if self.is_cycle_starting(count) {
            state.index = self.calc_index().unwrap();
            state.on = self.is_on(state);
        } else if self.is_cycle_finished(count) {
            state.on = false;
        }

        state.on_change = initial_on != state.on;
        state.index_change = initial_index != state.index;
    }

    #[inline(always)]
    fn calc_index(&mut self) -> Option<usize> {
        self.index = self.index.map_or(Option::Some(0), |index| {
            Option::Some((index + 1) % self.config.length().0 as usize)
        });
        self.index
    }

    #[inline(always)]
    fn is_cycle_starting(&self, count: u32) -> bool {
        count % self.cycle_target == 0
    }

    #[inline(always)]
    fn is_on(&self, state: &mut OutputState) -> bool {
        state.rng.rand_bool(self.config.prob()) && self.config.sequence()[state.index]
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
        let config = Config::new();
        let output = Output::new(1_920, &Tick::new(120), config.clone());

        let expected = Output {
            config,
            cycle_target: 1_920,
            index: Option::None,
            off_target: 960,
            resolution: 1_920,
        };

        assert_eq!(expected, output);
    }

    #[test]
    fn it_updates_on_through_two_full_cycles_at_pwm_p50() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let mut output = Output::new(1_920, &tick, Default::default());

        assert_eq!(OFF, state.on);

        output.tick(0, &mut state);
        assert_eq!(ON, state.on);

        output.tick(480, &mut state);
        assert_eq!(ON, state.on);

        output.tick(960, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_440, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920, &mut state);
        assert_eq!(ON, state.on);

        output.tick(2_400, &mut state);
        assert_eq!(ON, state.on);

        output.tick(2_880, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(3_360, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(3_840, &mut state);
        assert_eq!(ON, state.on);
    }

    #[test]
    fn it_updates_on_change_through_two_full_cycles_at_pwm_p50() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let mut output = Output::new(1_920, &tick, Default::default());

        assert_eq!(OFF, state.on_change);

        output.tick(0, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(1, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(2, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(959, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(960, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(961, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(1_919, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(1_920, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(1_921, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(2_879, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(2_880, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(2_881, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(3_839, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(3_840, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(3_841, &mut state);
        assert_eq!(OFF, state.on_change);
    }

    #[test]
    fn it_ticks_at_mult_two_point_zero_times_the_rate() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let rate = Rate::Mult(2, Frac::Zero);
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_rate(&tick, rate);

        assert_eq!(960, output.cycle_target);
        assert_eq!(480, output.off_target);
        assert_eq!(rate, output.config.rate());

        assert_eq!(OFF, state.on);

        output.tick(0, &mut state);
        assert_eq!(ON, state.on);

        output.tick(480, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(960, &mut state);
        assert_eq!(ON, state.on);

        output.tick(1_440, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920, &mut state);
        assert_eq!(ON, state.on);
    }

    #[test]
    fn it_ticks_at_div_five_point_one_third_the_rate() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let rate = Rate::Div(5, Frac::OneThird);
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_rate(&tick, rate);

        assert_eq!(5_120, output.off_target);
        assert_eq!(10_240, output.cycle_target);
        assert_eq!(rate, output.config.rate());

        assert_eq!(OFF, state.on);

        output.tick(0, &mut state);
        assert_eq!(ON, state.on);

        output.tick(5_119, &mut state);
        assert_eq!(ON, state.on);

        output.tick(5_120, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(10_239, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(10_240, &mut state);
        assert_eq!(ON, state.on);
    }

    #[test]
    fn it_skips_cycles_based_on_prob() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let prob = Prob::P10;
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_prob(prob);

        assert_eq!(OFF, state.on);

        output.tick(1_920, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(3_840, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(5_760, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(7_680, &mut state);
        assert_eq!(OFF, state.on);
    }

    #[test]
    fn it_works_with_pwm_pew() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let pwm = Pwm::Pew;
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_pwm(&tick, pwm);
        output.tick(1, &mut state);
    }

    #[test]
    fn it_updates_on_at_length_sixteen_at_density_four() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_output_type(&tick, OutputType::Euclid);

        output.tick(0, &mut state);
        assert_eq!(ON, state.on);

        output.tick(1_920, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 2, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 3, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 4, &mut state);
        assert_eq!(ON, state.on);

        output.tick(1_920 * 5, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 6, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 7, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 8, &mut state);
        assert_eq!(ON, state.on);

        output.tick(1_920 * 9, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 10, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 11, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 12, &mut state);
        assert_eq!(ON, state.on);

        output.tick(1_920 * 13, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 14, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 15, &mut state);
        assert_eq!(OFF, state.on);

        output.tick(1_920 * 16, &mut state);
        assert_eq!(ON, state.on);
    }

    #[test]
    fn it_updates_on_change_at_length_sixteen_at_density_four() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_output_type(&tick, OutputType::Euclid);

        assert_eq!(OFF, state.on_change);

        output.tick(0, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(1, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(39, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(40, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(1_919, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(1_920, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(1_959, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(3_839, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(3_840, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(3_879, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(5_759, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(5_760, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(5_799, &mut state);
        assert_eq!(OFF, state.on_change);

        output.tick(7_679, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(7_680, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(7_681, &mut state);
        assert_eq!(OFF, state.on_change);
        output.tick(7_719, &mut state);
        assert_eq!(ON, state.on_change);
        output.tick(7_720, &mut state);
        assert_eq!(OFF, state.on_change);
    }

    #[test]
    fn it_updates_index_at_length_sixteen_at_density_four() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_output_type(&tick, OutputType::Euclid);

        assert_eq!(0, state.index);

        output.tick(0, &mut state);
        assert_eq!(0, state.index);
        output.tick(1, &mut state);
        assert_eq!(0, state.index);
        output.tick(39, &mut state);
        assert_eq!(0, state.index);
        output.tick(40, &mut state);
        assert_eq!(0, state.index);

        output.tick(1_919, &mut state);
        assert_eq!(0, state.index);
        output.tick(1_920, &mut state);
        assert_eq!(1, state.index);
        output.tick(1_959, &mut state);
        assert_eq!(1, state.index);

        output.tick(3_839, &mut state);
        assert_eq!(1, state.index);
        output.tick(3_840, &mut state);
        assert_eq!(2, state.index);
        output.tick(3_879, &mut state);
        assert_eq!(2, state.index);

        output.tick(5_759, &mut state);
        assert_eq!(2, state.index);
        output.tick(5_760, &mut state);
        assert_eq!(3, state.index);
        output.tick(5_799, &mut state);
        assert_eq!(3, state.index);

        output.tick(7_679, &mut state);
        assert_eq!(3, state.index);
        output.tick(7_680, &mut state);
        assert_eq!(4, state.index);
        output.tick(7_681, &mut state);
        assert_eq!(4, state.index);
        output.tick(7_719, &mut state);
        assert_eq!(4, state.index);
        output.tick(7_720, &mut state);
        assert_eq!(4, state.index);
    }

    #[test]
    fn it_updates_index_change_at_length_sixteen_at_density_four() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let config = Config::new();
        let mut output = Output::new(1_920, &tick, config);
        output.set_output_type(&tick, OutputType::Euclid);

        assert_eq!(OFF, state.index_change);

        output.tick(0, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(1, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(39, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(40, &mut state);
        assert_eq!(OFF, state.index_change);

        output.tick(1_919, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(1_920, &mut state);
        assert_eq!(ON, state.index_change);
        output.tick(1_959, &mut state);
        assert_eq!(OFF, state.index_change);

        output.tick(3_839, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(3_840, &mut state);
        assert_eq!(ON, state.index_change);
        output.tick(3_879, &mut state);
        assert_eq!(OFF, state.index_change);

        output.tick(5_759, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(5_760, &mut state);
        assert_eq!(ON, state.index_change);
        output.tick(5_799, &mut state);
        assert_eq!(OFF, state.index_change);

        output.tick(7_679, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(7_680, &mut state);
        assert_eq!(ON, state.index_change);
        output.tick(7_681, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(7_719, &mut state);
        assert_eq!(OFF, state.index_change);
        output.tick(7_720, &mut state);
        assert_eq!(OFF, state.index_change);
    }

    #[test]
    fn it_updates_index_when_length_changes() {
        let mut state: OutputState = Default::default();
        let tick = Tick::new(120);
        let config = Config::new();
        let density = config.density();
        let mut output = Output::new(RESOLUTION, &tick, config);
        output.set_output_type(&tick, OutputType::Euclid);
        output.set_rate(&tick, Rate::Mult(2, Frac::Zero));

        const CYCLE_TARGET: u32 = 960;

        assert_eq!(0, state.index);

        output.tick(0, &mut state);
        assert_eq!(0, state.index);
        output.tick(CYCLE_TARGET, &mut state);
        assert_eq!(1, state.index);
        output.tick(CYCLE_TARGET * 2, &mut state);
        assert_eq!(2, state.index);

        output.set_sequence(Length(15), density);
        output.tick(CYCLE_TARGET * 3, &mut state);
        assert_eq!(3, state.index);
        output.tick(CYCLE_TARGET * 4, &mut state);
        assert_eq!(4, state.index);

        output.set_sequence(Length(16), density);
        output.tick(CYCLE_TARGET * 5, &mut state);
        assert_eq!(5, state.index);
        output.tick(CYCLE_TARGET * 6, &mut state);
        assert_eq!(6, state.index);
        output.tick(CYCLE_TARGET * 7, &mut state);
        assert_eq!(7, state.index);

        output.set_sequence(Length(15), density);
        output.tick(CYCLE_TARGET * 8, &mut state);
        assert_eq!(8, state.index);
        output.tick(CYCLE_TARGET * 9, &mut state);
        assert_eq!(9, state.index);
        output.tick(CYCLE_TARGET * 10, &mut state);
        assert_eq!(10, state.index);
        output.tick(CYCLE_TARGET * 11, &mut state);
        assert_eq!(11, state.index);
        output.tick(CYCLE_TARGET * 12, &mut state);
        assert_eq!(12, state.index);
        output.tick(CYCLE_TARGET * 13, &mut state);
        assert_eq!(13, state.index);
        output.tick(CYCLE_TARGET * 14, &mut state);
        assert_eq!(14, state.index);
        output.tick(CYCLE_TARGET * 15, &mut state);
        assert_eq!(0, state.index);
        output.tick(CYCLE_TARGET * 16, &mut state);
        assert_eq!(1, state.index);
        output.tick(CYCLE_TARGET * 17, &mut state);
        assert_eq!(2, state.index);
    }
}
