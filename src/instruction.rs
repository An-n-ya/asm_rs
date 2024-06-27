use crate::{opcode::OpCode, register::Register};

struct Instruction {
    prefix: Option<Prefix>,
    opcode: OpCode,
    operand1: Box<dyn Operand>,
    operand2: Box<dyn Operand>,
}

pub struct Prefix {}
pub trait Operand {
    fn is_register(&self) -> bool {
        false
    }
    fn is_memory(&self) -> bool {
        false
    }
    fn is_immediate(&self) -> bool {
        false
    }
    fn get_register(&self) -> Option<Register> {
        None
    }
}
