use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn part1(input: &mut dyn Read) -> String {
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut prev: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut res = 0;
    for line in lines {
        let curr = line.unwrap().parse().unwrap();
        if curr > prev {
            res += 1;
        }
        prev = curr;
    }

    res.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let mut a: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut b: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut c: u64 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut prev = a + b + c;

    let mut res = 0;

    for line in lines {
        let curr = line.unwrap().parse().unwrap();

        a = b;
        b = c;
        c = curr;

        let sum = a + b + c;
        if sum > prev {
            res += 1;
        }

        prev = sum;
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/01.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 7);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 5);
    }
}
