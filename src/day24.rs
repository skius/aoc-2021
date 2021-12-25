use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::Display;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Register::W => "w",
            Register::X => "x",
            Register::Y => "y",
            Register::Z => "z",
        })
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Value {
    Literal(i64),
    Register(Register),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(n) => write!(f, "{}", n),
            Value::Register(r) => write!(f, "{}", r),
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Register>() {
            Ok(r) => Ok(Value::Register(r)),
            Err(_) => Ok(Value::Literal(s.parse::<i64>().map_err(|_| ())?)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Ins {
    Input(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Eql(Register, Value),
}

thread_local! {
    static NUM_INPUTS: RefCell<usize> = RefCell::new(1);
}

impl Ins {
    fn print_progge(&self) {
        match self {
            Ins::Input(r) => {
                NUM_INPUTS.with(|n| {
                    let mut n = n.borrow_mut();
                    println!("let {} = arg{};", r, n);
                    *n += 1;
                });
            },
            Ins::Add(r, v) => println!("let {} = {} + {};", r, r, v),
            Ins::Mul(r, v) => println!("let {} = {} * {};", r, r, v),
            Ins::Div(r, v) => println!("let {} = {} / {};", r, r, v),
            Ins::Mod(r, v) => println!("let {} = {} % {};", r, r, v),
            Ins::Eql(r, v) => println!("let {} = int_of_bool({} == {});", r, r, v),
        }
    }
}

impl FromStr for Ins {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..3] == "inp" {
            return Ok(Ins::Input(s[4..].parse().unwrap()));
        }

        let reg = s[4..5].parse().unwrap();
        let val = s[6..].parse().unwrap();

        match &s[0..3] {
            "add" => Ok(Ins::Add(reg, val)),
            "mul" => Ok(Ins::Mul(reg, val)),
            "div" => Ok(Ins::Div(reg, val)),
            "mod" => Ok(Ins::Mod(reg, val)),
            "eql" => Ok(Ins::Eql(reg, val)),
            _ => Err(()),
        }
    }
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    for line in buf.lines() {
        line.parse::<Ins>().unwrap().print_progge()
        // println!("{:?}", );
    }

    todo!()
}

pub fn part2(_input: &mut dyn Read) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/24.txt");
    const REAL: &[u8] = include_bytes!("../inputs/24.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 590784);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 601104);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 2758514936282235usize);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 1262883317822267usize);
    }
}