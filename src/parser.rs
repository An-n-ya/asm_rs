use nom::{branch::alt, bytes::complete::tag, IResult};

use crate::register::{Len, Register};

fn register(input: &str) -> IResult<&str, Register> {
    let (input, res) = alt((ax, bx))(input)?;
    Ok((input, res))
}

fn ax(input: &str) -> IResult<&str, Register> {
    let (res, input) = alt((tag("rax"), tag("eax"), tag("ax"), tag("ah"), tag("al")))(input)?;
    let res = match res {
        "rax" => Register::AX(Len::Full),
        "eax" => Register::AX(Len::Low32),
        "ax" => Register::AX(Len::Low16),
        "ah" => Register::AX(Len::High8),
        "al" => Register::AX(Len::Low8),
        _ => unreachable!(),
    };
    Ok((input, res))
}
fn bx(input: &str) -> IResult<&str, Register> {
    let (res, input) = alt((tag("rbx"), tag("ebx"), tag("bx"), tag("bh"), tag("bl")))(input)?;
    let res = match res {
        "rbx" => Register::BX(Len::Full),
        "ebx" => Register::BX(Len::Low32),
        "bx" => Register::BX(Len::Low16),
        "bh" => Register::BX(Len::High8),
        "bl" => Register::BX(Len::Low8),
        _ => unreachable!(),
    };
    Ok((input, res))
}

// TODO: add macro to handle the parsing process
