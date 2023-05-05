use crate::prob::Prob;
use oorandom::Rand32;

const RNG_SEED: u64 = 0;

#[derive(Debug, PartialEq)]
pub struct Rng {
    prob: Prob,
    rng: Rand32,
}

impl Rng {
    pub fn new(prob: Prob) -> Self {
        Self {
            prob,
            rng: Rand32::new(RNG_SEED),
        }
    }
}
