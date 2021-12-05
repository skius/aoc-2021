use std::io::{BufRead, BufReader, Read};

struct Line {
    from: (usize, usize),
    to: (usize, usize),
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let mut lines = buf.lines();

    let segments = lines.map(|line| {
        let line = line.unwrap();
        let parts = line.split(" -> ").collect::<Vec<_>>();
        let from_x = parts[0].split(",").next().unwrap().parse::<usize>().unwrap();
        let from_y = parts[0].split(",").last().unwrap().parse::<usize>().unwrap();

        let to_x = parts[1].split(",").next().unwrap().parse::<usize>().unwrap();
        let to_y = parts[1].split(",").last().unwrap().parse::<usize>().unwrap();

        Line {
            from: (from_x, from_y),
            to: (to_x, to_y),
        }
    }).collect::<Vec<_>>();

    let mut grid = vec![vec![0usize; 1000]; 1000];

    for Line { from, to } in &segments {
        // println!("\nHandling line {:?} -> {:?}:", from, to);
        if from.0 == to.0 {
            for y in usize::min(from.1, to.1)..=usize::max(from.1, to.1) {
                // println!("Increasing ({}, {})", from.0, y);
                grid[from.0][y] += 1;
            }
        } else if from.1 == to.1 {
            for x in usize::min(from.0, to.0)..=usize::max(from.0, to.0) {
                // println!("Increasing ({}, {})", x, from.1);
                grid[x][from.1] += 1;
            }
        } else {


            // panic!("Invalid line {:?}, {:?}", from, to);
        }
    }

    let mut res = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] >= 2 {
                res += 1;
            }
        }
    }

    res.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let mut lines = buf.lines();

    let segments = lines.map(|line| {
        let line = line.unwrap();
        let parts = line.split(" -> ").collect::<Vec<_>>();
        let from_x = parts[0].split(",").next().unwrap().parse::<usize>().unwrap();
        let from_y = parts[0].split(",").last().unwrap().parse::<usize>().unwrap();

        let to_x = parts[1].split(",").next().unwrap().parse::<usize>().unwrap();
        let to_y = parts[1].split(",").last().unwrap().parse::<usize>().unwrap();

        Line {
            from: (from_x, from_y),
            to: (to_x, to_y),
        }
    }).collect::<Vec<_>>();

    let mut grid = vec![vec![0usize; 1000]; 1000];

    for Line { from, to } in &segments {
        // println!("\nHandling line {:?} -> {:?}:", from, to);
        if from.0 == to.0 {
            for y in usize::min(from.1, to.1)..=usize::max(from.1, to.1) {
                // println!("Increasing ({}, {})", from.0, y);
                grid[from.0][y] += 1;
            }
        } else if from.1 == to.1 {
            for x in usize::min(from.0, to.0)..=usize::max(from.0, to.0) {
                // println!("Increasing ({}, {})", x, from.1);
                grid[x][from.1] += 1;
            }
        } else {
            // Must be diagonal, and we want from to be the leftmost endpoint and to to be the rightmost endpoint
            let (from, to) = if from.0 <= to.0 {
                (from, to)
            } else {
                (to, from)
            };

            if from.1 < to.1 {
                // segment like so: \
                for x in from.0..=to.0 {
                    let y = from.1 + (x - from.0);
                    // println!("Increasing ({}, {})", x, y);
                    grid[x][y] += 1;
                }
            }

            if from.1 > to.1 {
                // segment like so: /
                for x in from.0..=to.0 {
                    let y = from.1 - (x - from.0);
                    // println!("Increasing ({}, {})", x, y);
                    grid[x][y] += 1;
                }
            }

            // panic!("Invalid line {:?}, {:?}", from, to);
        }
    }

    let mut res = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] >= 2 {
                res += 1;
            }
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/05.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 5);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 12);
    }
}

