use crate::instruction::Operand;

#[derive(Clone)]
pub struct Immediate(pub u64);

impl Operand for Immediate {
    fn is_immediate(&self) -> bool {
        true
    }
    fn get_immediate(&self) -> Option<Immediate> {
        Some(self.clone())
    }
}
