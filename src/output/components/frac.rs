#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Frac {
    Zero,
    OneThird,
    OneHalf,
    TwoThirds,
}

impl From<Frac> for f32 {
    fn from(val: Frac) -> Self {
        match val {
            Frac::Zero => 0.0,
            Frac::OneThird => 0.333_333_34,
            Frac::OneHalf => 0.5,
            Frac::TwoThirds => 0.666_666_7,
        }
    }
}
