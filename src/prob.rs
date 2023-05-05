use oorandom::Rand32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prob {
    P10,
    P20,
    P30,
    P40,
    P50,
    P60,
    P70,
    P80,
    P90,
    P100,
}

impl From<Prob> for u32 {
    fn from(val: Prob) -> Self {
        match val {
            Prob::P100 => 10,
            Prob::P90 => 9,
            Prob::P80 => 8,
            Prob::P70 => 7,
            Prob::P60 => 6,
            Prob::P50 => 5,
            Prob::P40 => 4,
            Prob::P30 => 3,
            Prob::P20 => 2,
            Prob::P10 => 1,
        }
    }
}

impl Prob {
    pub(crate) fn rand_bool(&self, mut rng: Rand32) -> bool {
        match self {
            Prob::P100 => true,
            &prob => {
                let target = prob.into();
                let result = rng.rand_range(1..11);
                result <= target
            }
        }
    }
}
