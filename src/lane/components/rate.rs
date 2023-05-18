#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rate {
    Div(u32, Frac),
    Unity,
    Mult(u32, Frac),
}

impl From<Rate> for f32 {
    fn from(val: Rate) -> Self {
        match val {
            Rate::Div(div, frac) => div as f32 + Into::<f32>::into(frac),
            Rate::Unity => 1.0,
            Rate::Mult(mult, frac) => (1.0 / mult as f32) + Into::<f32>::into(frac),
        }
    }
}

// Rate::Div(div, frac) => div * self.resolution,
// Rate::Unity => self.resolution,
// Rate::Mult(mult, frac) => ((1.0 / mult as f32) * self.resolution as f32) as
// u32,

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
