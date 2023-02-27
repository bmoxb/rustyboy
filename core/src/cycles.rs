use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};

#[derive(
    Debug, Display, Clone, Copy, PartialEq, PartialOrd, Add, AddAssign, Sub, SubAssign, From,
)]
pub struct MCycles(#[display(fmt = "{} m-cycles", _0)] pub u16);

#[derive(
    Debug, Display, Clone, Copy, PartialEq, PartialOrd, Add, AddAssign, Sub, SubAssign, From,
)]
pub struct TCycles(#[display(fmt = "{} t-cycles", _0)] pub u16);

impl From<MCycles> for TCycles {
    fn from(m: MCycles) -> Self {
        TCycles(m.0 * 4)
    }
}
