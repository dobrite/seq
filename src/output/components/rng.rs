use oorandom::Rand32;

use super::Prob;

const RNG_SEED: u64 = 0;

#[derive(Debug, PartialEq)]
pub struct Rng {
    rng: Rand32,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            rng: Rand32::new(RNG_SEED),
        }
    }

    pub fn rand_bool(&mut self, prob: Prob) -> bool {
        prob.rand_bool(&mut self.rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_randomly_chooses_a_bool_p100() {
        let mut rng = Rng::new();

        assert!(rng.rand_bool(Prob::P100));
    }

    #[test]
    fn it_randomly_chooses_a_bool_p50() {
        let mut rng = Rng::new();

        assert!(rng.rand_bool(Prob::P50));
    }

    #[test]
    fn it_randomly_chooses_a_bool_p10() {
        let mut rng = Rng::new();

        assert!(!rng.rand_bool(Prob::P10));
    }
}
