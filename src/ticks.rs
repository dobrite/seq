use fugit::RateExtU32;

const MICRO_SECONDS_PER_SECOND: u32 = 1_000_000;
type MicroSeconds = fugit::Duration<u64, 1, MICRO_SECONDS_PER_SECOND>;

const MAX_MULT: u32 = 192;
const PWM_PERCENT_INCREMENTS: u32 = 10;
const SECONDS_IN_MINUTES: f32 = 60.0;

pub fn resolution() -> u32 {
    PWM_PERCENT_INCREMENTS * MAX_MULT
}

pub fn tick_duration(bpm: f32) -> MicroSeconds {
    let bps = bpm / SECONDS_IN_MINUTES;
    const MULTIPLYER: f32 = (PWM_PERCENT_INCREMENTS * MAX_MULT) as f32;
    let hertz: u32 = (bps * MULTIPLYER) as u32;

    hertz.Hz::<1, 1>().into_duration().into()
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
        let result = tick_duration(10.0);

        assert_eq!(expected, result.ticks());
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_120_bpm() {
        let expected = 260;
        let result = tick_duration(120.0);

        assert_eq!(expected, result.ticks());
    }

    #[test]
    fn it_computes_tick_duration_in_millis_for_300_bpm() {
        let expected = 104;
        let result = tick_duration(300.0);

        assert_eq!(expected, result.ticks());
    }
}
