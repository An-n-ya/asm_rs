extern crate proc_macro;

use macros::register_macro;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while},
    combinator::map_res,
    error::Error,
    sequence::preceded,
    IResult,
};

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

fn digits(input: &str) -> IResult<&str, u64> {
    alt((binary_digits, hex_digits, decimal_digits))(input)
}

fn binary_digits(input: &str) -> IResult<&str, u64> {
    map_res(preceded(tag("0b"), take_while(is_binary)), from_binary)(input)
}
fn hex_digits(input: &str) -> IResult<&str, u64> {
    map_res(preceded(tag("0x"), take_while(is_hex_digit)), from_hex)(input)
}
fn decimal_digits(input: &str) -> IResult<&str, u64> {
    map_res(take_while(is_digit), from_decimal)(input)
}
fn is_binary(c: char) -> bool {
    c.is_digit(2)
}
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn from_decimal(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 10)
}
fn from_binary(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 2)
}
fn from_hex(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 16)
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
    #[test]
    fn test_digits_parse() {
        let inputs = ["0xdeadbeef", "31415926", "0b10010"];
        let expectes = [0xdeadbeef, 31415926, 0b10010];
        for (input, expect) in inputs.iter().zip(expectes.iter()) {
            let (remain, reg) = digits(input).expect("cannot parse rax");
            assert_eq!(reg, *expect);
            assert!(remain.len() == 0);
        }
    }
}
