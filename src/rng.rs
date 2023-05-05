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

    pub fn rand_bool(&mut self) -> bool {
        self.prob.rand_bool(&mut self.rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_randomly_chooses_a_bool_p100() {
        let mut rng = Rng::new(Prob::P100);

        assert!(rng.rand_bool());
    }

    #[test]
    fn it_randomly_chooses_a_bool_p50() {
        let mut rng = Rng::new(Prob::P50);

        assert!(rng.rand_bool());
    }

    #[test]
    fn it_randomly_chooses_a_bool_p10() {
        let mut rng = Rng::new(Prob::P10);

        assert!(!rng.rand_bool());
    }
}
