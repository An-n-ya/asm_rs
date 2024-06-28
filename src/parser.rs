extern crate proc_macro;

use macros::register_macro;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take, take_while},
    character::complete::space0,
    combinator::map_res,
    error::{Error, ErrorKind},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{
    immediate::Immediate,
    instruction::Operand,
    memory::{Memory, Offset},
    register::{Len, Register},
};
use nom::Err;

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
    let (input, res) = delimited(
        space0,
        alt((rax, rbx, rdx, rbx, rsp, rbp, rsi, rdi, seg_register)),
        space0,
    )(input)?;
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
    delimited(space0, delimited(tag("["), memory_inner, tag("]")), space0)(input)
}
fn memory_inner(input: &str) -> IResult<&str, OperandBox> {
    alt((memory_type1, memory_type2, memory_type3))(input)
}
fn memory_type3(input: &str) -> IResult<&str, OperandBox> {
    let (input, res) = immediate(input)?;
    let offset = res.get_immediate().unwrap();
    let res = if let Ok(offset) = offset.try_into() {
        Box::new(Memory::Offset(offset))
    } else {
        return Err(nom::Err::Error(Error::new(
            "offset too large",
            nom::error::ErrorKind::Verify,
        )));
    };
    Ok((input, res))
}
fn memory_type2(input: &str) -> IResult<&str, OperandBox> {
    map_res(
        tuple((register, tag("+"), immediate)),
        |(reg1, _, offset)| {
            let reg1 = reg1.get_register().unwrap();
            let offset = offset.get_immediate().unwrap();
            memory_from_type2(reg1, offset)
        },
    )(input)
}
fn memory_type1(input: &str) -> IResult<&str, OperandBox> {
    map_res(
        tuple((register, tag("+"), register, tag("+"), immediate)),
        |(reg1, _, reg2, _, offset)| {
            let reg1 = reg1.get_register().unwrap();
            let reg2 = reg2.get_register().unwrap();
            let offset = offset.get_immediate().unwrap();
            memory_from_type1(reg1, reg2, offset)
        },
    )(input)
}
fn memory_from_type2(
    reg: Register,
    offset: Immediate,
) -> Result<OperandBox, Err<Error<&'static str>>> {
    if let Ok(offset) = offset.try_into() {
        match reg {
            Register::BX(Len::Low16) => Ok(Box::new(Memory::BX(offset))),
            Register::BP(Len::Low16) => Ok(Box::new(Memory::BP(offset))),
            Register::SI(Len::Low16) => Ok(Box::new(Memory::SI(offset))),
            Register::DI(Len::Low16) => Ok(Box::new(Memory::DI(offset))),
            _ => Err(nom::Err::Error(Error::new(
                "reg1 must be bx/bp/si/di ",
                nom::error::ErrorKind::Verify,
            ))),
        }
    } else {
        return Err(nom::Err::Error(Error::new(
            "offset too large",
            nom::error::ErrorKind::Verify,
        )));
    }
}
fn memory_from_type1(
    reg1: Register,
    reg2: Register,
    offset: Immediate,
) -> Result<OperandBox, Err<Error<&'static str>>> {
    if let Ok(offset) = offset.try_into() {
        match reg1 {
            Register::BX(Len::Low16) => match reg2 {
                Register::SI(Len::Low16) => Ok(Box::new(Memory::BXSI(offset))),
                Register::DI(Len::Low16) => Ok(Box::new(Memory::BXDI(offset))),
                _ => Err(nom::Err::Error(Error::new(
                    "reg2 must be si or di",
                    nom::error::ErrorKind::Verify,
                ))),
            },
            Register::BP(Len::Low16) => match reg2 {
                Register::SI(Len::Low16) => Ok(Box::new(Memory::BPSI(offset))),
                Register::DI(Len::Low16) => Ok(Box::new(Memory::BPDI(offset))),
                _ => Err(nom::Err::Error(Error::new(
                    "reg2 must be si or di",
                    nom::error::ErrorKind::Verify,
                ))),
            },
            _ => Err(nom::Err::Error(Error::new(
                "reg1 must be bx or bp",
                nom::error::ErrorKind::Verify,
            ))),
        }
    } else {
        return Err(nom::Err::Error(Error::new(
            "offset too large",
            nom::error::ErrorKind::Verify,
        )));
    }
}

pub fn immediate(input: &str) -> IResult<&str, OperandBox> {
    let (input, res) = delimited(space0, digits, space0)(input)?;
    Ok((input, Box::new(Immediate(res))))
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
        let inputs = [" rax ", " ss ", "bx"];
        let expectes = [
            Register::AX(Len::Full),
            Register::SS,
            Register::BX(Len::Low16),
        ];
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
    #[test]
    fn test_memory_parse() {
        let inputs = ["[0x8000]", "[bx + 0x1]", "[bx + si + 0b10010]"];
        let expectes = [
            Memory::Offset(Offset::U16(0x8000)),
            Memory::BX(Offset::U8(1)),
            Memory::BXSI(Offset::U8(0b10010)),
        ];
        for (input, expect) in inputs.iter().zip(expectes.iter()) {
            let (remain, reg) = memory(input).expect(&format!("cannot parse {}", input));
            assert_eq!(reg.get_memory().unwrap(), *expect);
            assert!(remain.len() == 0);
        }
    }
}
