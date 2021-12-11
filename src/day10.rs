use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Kind {
    Square, // []
    Round, // ()
    Curly, // {}
    Angle, // <>
}

impl Kind {
    fn syntax_score(&self) -> usize {
        match self {
            Kind::Square => 57,
            Kind::Round => 3,
            Kind::Curly => 1197,
            Kind::Angle => 25137,
        }
    }

    fn auto_score(&self) -> usize {
        match self {
            Kind::Square => 2,
            Kind::Round => 1,
            Kind::Curly => 3,
            Kind::Angle => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Open(Kind),
    Closed(Kind),
}

fn get_inputs(input: &mut dyn Read) -> Vec<Vec<Direction>> {
    let mut buf = BufReader::new(input);
    let mut lines = buf.lines();
    lines.map(|l| {
        let l = l.unwrap();
        l.chars().map(|c| {
            match c {
                '<' => Direction::Open(Kind::Angle),
                '>' => Direction::Closed(Kind::Angle),
                '(' => Direction::Open(Kind::Round),
                ')' => Direction::Closed(Kind::Round),
                '{' => Direction::Open(Kind::Curly),
                '}' => Direction::Closed(Kind::Curly),
                '[' => Direction::Open(Kind::Square),
                ']' => Direction::Closed(Kind::Square),
                _ => panic!("Unknown character: {}", c),
            }
        }).collect()
    }).collect()
}

pub fn part1(input: &mut dyn Read) -> String {
    let stacks = get_inputs(input);

    let mut total = 0;
    'outer: for line in stacks {
        let mut stack = Vec::new();
        for dir in line {
            match dir {
                Direction::Open(kind) => {
                    stack.push(kind);
                },
                Direction::Closed(kind) => {
                    if stack.pop().unwrap() != kind {
                        total += kind.syntax_score();
                        continue 'outer;
                    }
                },
            }
        }
    }

    total.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let stacks = get_inputs(input);

    let mut totals = vec![];
    'outer: for line in stacks {
        let mut stack = Vec::new();
        for dir in line {
            match dir {
                Direction::Open(kind) => {
                    stack.push(kind);
                },
                Direction::Closed(kind) => {
                    if stack.pop().unwrap() != kind {
                        // ignore corrupt lines
                        continue 'outer;
                    }
                },
            }
        }

        let mut total = 0;
        // our stack contains open kinds, and to close them, we must have closing kinds but _in reverse_
        for elem in stack.iter().rev() {
            total *= 5;
            total += elem.auto_score();
        }

        totals.push(total);
    }

    let len = totals.len();
    totals.select_nth_unstable(len / 2).1.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/10.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 26397);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 288957);
    }
}