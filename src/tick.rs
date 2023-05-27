const MAX_MULT: u32 = 192;
const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: f32 = 60.0;
const MICRO_SECONDS_PER_SECOND: f32 = 1_000_000.0;
pub const RESOLUTION: u32 = PWM_PERCENT_INCREMENTS * MAX_MULT;

pub struct Tick {
    pub bpm: u32,
    pub count: u32,
    pub duration_micros: u64,
}

impl Default for Tick {
    fn default() -> Self {
        Self::new(120)
    }
}

impl Tick {
    pub fn new(bpm: u32) -> Self {
        let mut tick = Self {
            count: 0,
            bpm,
            duration_micros: 0,
        };

        tick.set_bpm(bpm);
        tick
    }

    pub fn set_bpm(&mut self, bpm: u32) {
        self.bpm = bpm;
        self.duration_micros = self.duration_micros(bpm);
    }

    fn duration_micros(&self, bpm: u32) -> u64 {
        let beats_per_minute = bpm as f32;
        let beats_per_second = beats_per_minute / SECONDS_IN_MINUTES;
        let ticks_per_second = beats_per_second * RESOLUTION as f32;
        round(MICRO_SECONDS_PER_SECOND / ticks_per_second) as u64
    }
}

#[inline(always)]
fn round(val: f32) -> f32 {
    let floor = val as u32 as f32;

    if val - floor < 0.5 {
        floor
    } else {
        floor + 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_resolution() {
        let expected = 1_920;
        let result = RESOLUTION;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_10_bpm() {
        let expected = 3_125;
        let tick = Tick::new(10);
        let result = tick.duration_micros;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_120_bpm() {
        let expected = 260;
        let tick = Tick::new(120);
        let result = tick.duration_micros;

        assert_eq!(expected, result);
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_300_bpm() {
        let expected = 104;
        let tick = Tick::new(300);
        let result = tick.duration_micros;

        assert_eq!(expected, result);
    }
}
