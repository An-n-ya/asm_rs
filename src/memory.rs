use nom::error::Error;

use crate::{immediate::Immediate, instruction::Operand};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Memory {
    BXSI(Offset),
    BXDI(Offset),
    BPSI(Offset),
    BPDI(Offset),
    SI(Offset),
    DI(Offset),
    BP(Offset),
    BX(Offset),
    Offset(Offset),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Offset {
    U16(u16),
    U8(u8),
    None,
}

impl TryFrom<Immediate> for Offset {
    type Error = Error<&'static str>;

    fn try_from(value: Immediate) -> Result<Self, Self::Error> {
        let value = value.0;
        if value > u16::MAX as u64 {
            Err(Error::new(
                "offset too large",
                nom::error::ErrorKind::TooLarge,
            ))
        } else if value > u8::MAX as u64 {
            Ok(Offset::U16(value as u16))
        } else {
            Ok(Offset::U8(value as u8))
        }
    }
}

impl Operand for Memory {
    fn is_memory(&self) -> bool {
        true
    }
    fn get_memory(&self) -> Option<Memory> {
        Some(self.clone())
    }
}
