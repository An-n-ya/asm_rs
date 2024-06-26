extern crate proc_macro;

use macros::register_macro;
use nom::{branch::alt, IResult};

use crate::register::Register;

register_macro!(["rax", "eax", "ax", "ah", "al"]);
register_macro!(["rcx", "ecx", "cx", "ch", "cl"]);
register_macro!(["rdx", "edx", "dx", "dh", "dl"]);
register_macro!(["rbx", "ebx", "bx", "bh", "bl"]);
register_macro!(["rsp", "esp", "sp"]);
register_macro!(["rbp", "ebp", "bp"]);
register_macro!(["rsi", "esi", "si"]);
register_macro!(["rdi", "edi", "di"]);

pub fn register(input: &str) -> IResult<&str, Register> {
    let (input, res) = alt((rax, rbx, rdx, rbx, rsp, rbp, rsi, rdi))(input)?;
    Ok((input, res))
}

#[cfg(test)]
mod tests {
    use crate::register::Len;

    use super::*;

    #[test]
    fn test_register_parse() {
        let input = "rax";
        let (remain, reg) = register(input).expect("cannot parse rax");
        assert_eq!(reg, Register::AX(Len::Full));
        assert!(remain.len() == 0);
    }
}
