#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pwm {
    P(u8),
    Pew,
}

impl Pwm {
    pub fn index(&self) -> usize {
        match self {
            Pwm::Pew => 0,
            Pwm::P(10) => 1,
            Pwm::P(20) => 2,
            Pwm::P(30) => 3,
            Pwm::P(40) => 4,
            Pwm::P(50) => 5,
            Pwm::P(60) => 6,
            Pwm::P(70) => 7,
            Pwm::P(80) => 8,
            Pwm::P(90) => 9,
            _ => unreachable!(),
        }
    }
}
