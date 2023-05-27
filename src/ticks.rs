const MAX_MULT: u32 = 192;
const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: f32 = 60.0;
const MICRO_SECONDS_PER_SECOND: f32 = 1_000_000.0;

pub fn resolution() -> u32 {
    PWM_PERCENT_INCREMENTS * MAX_MULT
}

pub fn tick_duration_micros(bpm: f32) -> u64 {
    let beats_per_second = bpm / SECONDS_IN_MINUTES;
    let ticks_per_second = beats_per_second * resolution() as f32;
    round(MICRO_SECONDS_PER_SECOND / ticks_per_second) as u64
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
        let result = resolution();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_10_bpm() {
        let expected = 3_125;
        let result = tick_duration_micros(10.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_120_bpm() {
        let expected = 260;
        let result = tick_duration_micros(120.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_300_bpm() {
        let expected = 104;
        let result = tick_duration_micros(300.0);

        assert_eq!(expected, result);
    }
}
