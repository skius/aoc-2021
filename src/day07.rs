use std::io::Read;

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let positions = buf.trim().split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let sum: usize = positions.iter().sum();
    let avg = (sum as f64) / (positions.len() as f64);
    let avg = avg.round() as usize;
    // println!("avg: {}", avg);

    let mut total = usize::MAX;

    for i in 0..=*positions.iter().max().unwrap() {
        // let mut curr_total = 0;
        let curr_total = positions.iter().map(|&pos| (pos as isize - (i as isize)).abs() as usize).sum();
        if curr_total < total {
            total = curr_total;
        }
    }


    total.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let positions = buf.trim().split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let sum: usize = positions.iter().sum();
    let avg = (sum as f64) / (positions.len() as f64);
    let avg = avg.round() as usize;
    // println!("avg: {}", avg);

    let mut total = usize::MAX;

    for i in 0..=*positions.iter().max().unwrap() {
        // let mut curr_total = 0;
        let curr_total = positions.iter().map(|&pos| {
            let n = (pos as isize - (i as isize)).abs() as usize;
            (n * (n + 1)) / 2
        }).sum();
        if curr_total < total {
            total = curr_total;
        }
    }


    total.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/07.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 37);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 168);
    }
}