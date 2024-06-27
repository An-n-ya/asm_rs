extern crate proc_macro;

use macros::register_macro;
use nom::{branch::alt, IResult};

use crate::instruction::Operand;

register_macro!(["rax", "eax", "ax", "ah", "al"]);
register_macro!(["rcx", "ecx", "cx", "ch", "cl"]);
register_macro!(["rdx", "edx", "dx", "dh", "dl"]);
register_macro!(["rbx", "ebx", "bx", "bh", "bl"]);
register_macro!(["rsp", "esp", "sp"]);
register_macro!(["rbp", "ebp", "bp"]);
register_macro!(["rsi", "esi", "si"]);
register_macro!(["rdi", "edi", "di"]);

pub fn register(input: &str) -> IResult<&str, Box<dyn Operand>> {
    let (input, res) = alt((rax, rbx, rdx, rbx, rsp, rbp, rsi, rdi))(input)?;
    Ok((input, Box::new(res)))
}

#[cfg(test)]
mod tests {
    use crate::register::Len;
    use crate::register::Register;

    use super::*;

    #[test]
    fn test_register_parse() {
        let input = "rax";
        let (remain, reg) = register(input).expect("cannot parse rax");
        assert_eq!(reg.get_register().unwrap(), Register::AX(Len::Full));
        assert!(remain.len() == 0);
    }
}
