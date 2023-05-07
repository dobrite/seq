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
}
