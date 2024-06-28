use crate::instruction::Operand;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Register {
    AX(Len),
    CX(Len),
    DX(Len),
    BX(Len),
    SP(Len),
    BP(Len),
    SI(Len),
    DI(Len),
    CS,
    SS,
    DS,
    ES,
    FS,
    GS
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Len {
    Low8,
    High8,
    Low16,
    Low32,
    Full,
}

impl Operand for Register {
    fn is_register(&self) -> bool {
        true
    }

    fn get_register(&self) -> Option<Register> {
        Some(self.clone())
    }
}
