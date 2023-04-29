#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rate {
    Div(u8),
    Unity,
    Mult(u8),
}
