use super::Frac;

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
