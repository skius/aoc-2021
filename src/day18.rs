use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read};
use std::iter::Peekable;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::time::Instant;

#[derive(Clone)]
enum Value {
    Number(i64),
    Pair(Box<Pair>),
}
#[derive(Clone)]
struct Pair(Value, Value);

impl Value {
    fn force_number(&self) -> i64 {
        match self {
            Value::Number(n) => *n,
            Value::Pair(p) => panic!("not a regular number: {:?}", p),
        }
    }

    fn find_first_right(&mut self) -> Option<&mut Value> {
        match self {
            Value::Pair(p) => {
                p.deref_mut().0.find_first_right()
            }
            Value::Number(_) => Some(self),
        }
    }

    fn find_first_left(&mut self) -> Option<&mut Value> {
        match self {
            Value::Pair(p) => {
                p.deref_mut().1.find_first_left()
            }
            Value::Number(_) => Some(self),
        }
    }

    fn explode(&mut self, depth: usize, left_num: Option<&mut Value>, right_num: Option<&mut Value>) -> bool {
        if depth == 4 {
            if let Value::Pair(p) = self {
                // explode
                let Pair(left, right) = *p.deref().clone();
                let left = left.force_number();
                let right = right.force_number();

                // add left to left_regular_number
                if let Some(Value::Number(num)) = left_num {
                    *num += left;
                }
                if let Some(Value::Number(num)) = right_num {
                    *num += right;
                }

                *self = Value::Number(0);
                return true;
            }
            // Nested regular number can be ignored
            return false;
        }

        if let Value::Pair(ref mut inner) = self {
            let pair = inner.deref_mut();
            if pair.0.explode(depth + 1, left_num, pair.1.find_first_right()) {
                return true;
            }
            if pair.1.explode(depth + 1, pair.0.find_first_left(), right_num) {
                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        match self {
            Value::Pair(p) => {
                let Pair(left, right) = p.deref_mut();
                if left.split() {
                    return true;
                }
                if right.split() {
                    return true;
                }
                return false;
            }
            Value::Number(num) => {
                if *num < 10 {
                    return false;
                }

                let new_left = Value::Number(*num / 2);
                let new_right = Value::Number((*num + 1) / 2);
                *self = Value::Pair(Box::new(Pair(new_left, new_right)));
                return true;
            }
        }
    }

    fn reduce(&mut self) {
        if self.explode(0, None, None) {
            // Something changed, start reduction process again
            // println!("Exploded, new value: {:?}", self);
            self.reduce();
            return;
        }
        // Otherwise, check splittable numbers
        if self.split() {
            // Something changed, start reduction process again
            // println!("Split, new value: {:?}", self);
            self.reduce();
            return;
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Value::Number(n) => *n,
            Value::Pair(p) => {
                let Pair(left, right) = p.deref();
                3 * left.magnitude() + 2 * right.magnitude()
            }
        }
    }

    fn add(self, b: Value) -> Self {
        Value::Pair(Box::new(Pair(self, b)))
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Pair(p) => write!(f, "{:?}", p),
        }
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.0, self.1)
    }
}

fn parse_number(s: &mut Peekable<impl Iterator<Item = char>>) -> i64 {
    let mut result = 0;
    while let Some(c) = s.peek() {
        if c.is_numeric() {
            result = result * 10 + c.to_digit(10).unwrap() as i64;
            s.next();
        } else {
            break;
        }
    }
    result
}

fn parse_value(s: &mut Peekable<impl Iterator<Item = char>>) -> Value {
    if s.peek().unwrap() == &'[' {
        Value::Pair(Box::new(parse_pair(s)))
    } else {
        Value::Number(parse_number(s))
    }
}

fn parse_pair(s: &mut Peekable<impl Iterator<Item = char>>) -> Pair {
    let open = s.next().unwrap();
    assert!(open == '[');

    let first = parse_value(s);
    let comma = s.next().unwrap();
    assert!(comma == ',');
    let second = parse_value(s);
    let close = s.next().unwrap();
    assert!(close == ']');
    Pair(first, second)
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_pair(&mut s.chars().peekable()))
    }
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let pairs = buf.lines().map(|l| {
        let line = l.unwrap();
        Value::Pair(Box::new(line.parse().unwrap()))
    }).collect::<Vec<Value>>();

    // println!("{:#?}", pairs);

    let sum = pairs.into_iter().reduce(|a, b| {
        let mut sum = a.add(b);
        // println!("\nIntermediate sum: {:?}", sum);
        sum.reduce();
        sum
    }).unwrap();
    // println!("Final: {:?}", sum);

    sum.magnitude().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    // let pre = Instant::now();
    let mut buf = BufReader::new(input);
    let pairs = buf.lines().map(|l| {
        let line = l.unwrap();
        Value::Pair(Box::new(line.parse().unwrap()))
    }).collect::<Vec<Value>>();
    // println!("Parsed input in {:?}", pre.elapsed());

    // let pre = Instant::now();
    let mut max = 0;
    for p1_i in 0..pairs.len() {
        for p2_i in 0..pairs.len() {
            if p1_i == p2_i {
                continue;
            }
            let p1 = pairs[p1_i].clone();
            let p2 = pairs[p2_i].clone();
            let mut sum = p1.add(p2);
            sum.reduce();
            let magn = sum.magnitude();
            if magn > max {
                max = magn;
            }
        }
    }
    // println!("Found max in {:?}", pre.elapsed());


    max.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/18.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/18.2.txt");
    const REAL: &[u8] = include_bytes!("../inputs/18.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 3488);
    }

    #[test]
    fn sample2_part1() {
        test_implementation(part1, SAMPLE2, 4140);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 4235);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE2, 3993);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 4659);
    }
}