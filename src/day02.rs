use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;

enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}
use Command::*;

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "up" => Ok(Up(parts[1].parse().unwrap())),
            "down" => Ok(Down(parts[1].parse().unwrap())),
            "forward" => Ok(Forward(parts[1].parse().unwrap())),
            _ => unreachable!(),
        }
    }
}

pub fn part1(input: &mut dyn Read) -> String {
    let reader = BufReader::new(input);
    let (h, d) = reader
        .lines()
        .map(|l| l.unwrap().parse::<Command>().unwrap())
        .fold((0, 0), |(p_h, p_d), c| {
            match c {
                Forward(n) => (p_h + n, p_d),
                Up(d) => (p_h, p_d - d),
                Down(d) => (p_h, p_d + d),
            }
        });

    (h * d).to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let reader = BufReader::new(input);
    let (h, d, _a) = reader
        .lines()
        .map(|l| l.unwrap().parse::<Command>().unwrap())
        .fold((0, 0, 0), |(p_h, p_d, p_a), c| {
            match c {
                Forward(n) => (p_h + n, p_d + n * p_a, p_a),
                Up(d) => (p_h, p_d, p_a - d),
                Down(d) => (p_h, p_d, p_a + d),
            }
        });

    (h * d).to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/02.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 150);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 900);
    }
}
