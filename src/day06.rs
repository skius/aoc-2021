use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::ops::Deref;

fn after_days(input: &mut dyn Read, days: usize) -> String {
    let buf = BufReader::new(input);
    let timers = buf.split(b',').map(|s| {
        let num = String::from_utf8(s.unwrap()).unwrap().trim().to_string();
        let num = num.parse::<usize>().unwrap();

        num
    }).collect::<Vec<_>>();
    let mut pop: HashMap<usize, usize> = HashMap::new();
    for timer in &timers {
        *pop.entry(*timer).or_insert(0) += 1;
    }
    println!("timers: {:?}", &timers);
    println!("In the beginning (before day 1):\n{:?}", pop);

    for i in 0..days {
        let mut new_pop: HashMap<usize, usize> = HashMap::new();
        for timer in 1..=8 {
            new_pop.insert(timer-1, pop.get(&timer).copied().unwrap_or(0));
        }

        new_pop.insert(8, pop.get(&0).copied().unwrap_or(0));
        *new_pop.entry(6).or_insert(0) += pop.get(&0).copied().unwrap_or(0);
        pop = new_pop;
        println!("After {} days:", i+1);
        println!("{:?}\n", pop);
    }

    pop.iter().map(|(k ,v)| v).sum::<usize>().to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    after_days(input, 80)

}

pub fn part2(input: &mut dyn Read) -> String {
    after_days(input, 256)
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/06.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 5934usize);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 26984457539usize);
    }
}