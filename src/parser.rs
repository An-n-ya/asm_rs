extern crate proc_macro;

use macros::register_macro;
use nom::{branch::alt, bytes::complete::tag, error::Error, IResult};

use crate::{instruction::Operand, register::Register};

register_macro!(["rax", "eax", "ax", "ah", "al"]);
register_macro!(["rcx", "ecx", "cx", "ch", "cl"]);
register_macro!(["rdx", "edx", "dx", "dh", "dl"]);
register_macro!(["rbx", "ebx", "bx", "bh", "bl"]);
register_macro!(["rsp", "esp", "sp"]);
register_macro!(["rbp", "ebp", "bp"]);
register_macro!(["rsi", "esi", "si"]);
register_macro!(["rdi", "edi", "di"]);

type OperandBox = Box<dyn Operand>;

pub fn register(input: &str) -> IResult<&str, OperandBox> {
    let (input, res) = alt((rax, rbx, rdx, rbx, rsp, rbp, rsi, rdi, seg_register))(input)?;
    Ok((input, Box::new(res)))
}

fn seg_register(input: &str) -> IResult<&str, Register> {
    let (input, res) = alt((
        tag("ss"),
        tag("cs"),
        tag("ds"),
        tag("es"),
        tag("gs"),
        tag("fs"),
    ))(input)?;
    let res = match res {
        "ss" => Register::SS,
        "cs" => Register::CS,
        "ds" => Register::DS,
        "es" => Register::ES,
        "gs" => Register::GS,
        "fs" => Register::FS,
        _ => {
            return Err(nom::Err::Error(Error::new(
                "cannot parse seg register",
                nom::error::ErrorKind::Tag,
            )));
        }
    };
    Ok((input, res))
}

pub fn memory(input: &str) -> IResult<&str, OperandBox> {
    unimplemented!()
}
pub fn immediate(input: &str) -> IResult<&str, OperandBox> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::register::Len;
    use crate::register::Register;

    use super::*;

    #[test]
    fn test_register_parse() {
        let inputs = ["rax", "ss"];
        let expectes = [Register::AX(Len::Full), Register::SS];
        for (input, expect) in inputs.iter().zip(expectes.iter()) {
            let (remain, reg) = register(input).expect("cannot parse rax");
            assert_eq!(reg.get_register().unwrap(), *expect);
            assert!(remain.len() == 0);
        }
    }
}
