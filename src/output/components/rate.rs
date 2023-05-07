#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rate {
    Div(f32),
    Unity,
    Mult(f32),
}
