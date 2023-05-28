use crate::{math, tick::Tick};

const PEW_MODE_IN_MICRO_SECONDS: u64 = 10_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pwm {
    Pew,
    P10,
    P20,
    P30,
    P40,
    P50,
    P60,
    P70,
    P80,
    P90,
}

impl From<Pwm> for f32 {
    fn from(val: Pwm) -> Self {
        match val {
            Pwm::Pew => 0.1,
            Pwm::P10 => 0.1,
            Pwm::P20 => 0.2,
            Pwm::P30 => 0.3,
            Pwm::P40 => 0.4,
            Pwm::P50 => 0.5,
            Pwm::P60 => 0.6,
            Pwm::P70 => 0.7,
            Pwm::P80 => 0.8,
            Pwm::P90 => 0.9,
        }
    }
}

impl Pwm {
    pub fn index(&self) -> usize {
        match self {
            Pwm::Pew => 0,
            Pwm::P10 => 1,
            Pwm::P20 => 2,
            Pwm::P30 => 3,
            Pwm::P40 => 4,
            Pwm::P50 => 5,
            Pwm::P60 => 6,
            Pwm::P70 => 7,
            Pwm::P80 => 8,
            Pwm::P90 => 9,
        }
    }

    pub fn off_target(&self, tick: &Tick, cycle_target: u32) -> u32 {
        match self {
            Pwm::Pew => self.calculate_pew_mode_off_target(tick),
            _ => {
                let ratio: f32 = (*self).into();
                (ratio * cycle_target as f32) as u32
            }
        }
    }

    fn calculate_pew_mode_off_target(&self, tick: &Tick) -> u32 {
        math::ceil(PEW_MODE_IN_MICRO_SECONDS as f32 / tick.duration_micros as f32) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tick::RESOLUTION;

    #[test]
    fn it_calcs_off_target_for_p10() {
        let result = Pwm::P10.off_target(&Tick::new(120), RESOLUTION);
        let expected = 192;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_calcs_off_target_for_p50() {
        let result = Pwm::P50.off_target(&Tick::new(120), RESOLUTION);
        let expected = 960;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_calcs_off_target_for_p100() {
        let result = Pwm::P90.off_target(&Tick::new(120), RESOLUTION);
        let expected = 1_728;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_calcs_off_target_for_pew_mode_bpm_10() {
        let result = Pwm::Pew.off_target(&Tick::new(10), RESOLUTION);
        let expected = 4;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_calcs_off_target_for_pew_mode_bpm_120() {
        let result = Pwm::Pew.off_target(&Tick::new(120), RESOLUTION);
        let expected = 39;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_calcs_off_target_for_pew_mode_bpm_300() {
        let result = Pwm::Pew.off_target(&Tick::new(300), RESOLUTION);
        let expected = 97;

        assert_eq!(expected, result);
    }
}
