use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read};

pub fn part1(input: &mut dyn Read) -> String {
    let mut reader = BufReader::new(input);

    let mut cave = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        cave.push(
            line.chars()
                .map(|c| c as u8 - b'0')
                .collect::<Vec<u8>>()
        );
    }

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    let target_x = cave[0].len() - 1;
    let target_y = cave.len() - 1;


    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    costs.insert((0, 0), 0);
    let mut worklist = VecDeque::new();
    worklist.extend([(0,1), (1,0)]);

    while !worklist.is_empty() {
        let (curr_x, curr_y) = worklist.pop_front().unwrap();
        let curr_cost = *costs.get(&(curr_x, curr_y)).unwrap_or(&usize::MAX);

        let mut new_cost = cave[curr_y][curr_x] as usize;
        // add minimum of surrounding costs

        new_cost += adjacents(&cave, curr_x, curr_y).map(|(x, y)| {
            *costs.get(&(x, y)).unwrap_or(&usize::MAX)
        }).min().unwrap();

        if new_cost < curr_cost {
            costs.insert((curr_x, curr_y), new_cost);
            for (new_x, new_y) in adjacents(&cave, curr_x, curr_y) {
                worklist.push_back((new_x, new_y));
            }
        }
    }

    costs[&(target_x, target_y)].to_string()
}

fn adjacents(cave: &Vec<Vec<u8>>, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut res = Vec::with_capacity(4);
    if 0 < x {
        res.push((x - 1, y));
    }
    if x < cave[0].len() - 1 {
        res.push((x + 1, y));
    }
    if 0 < y {
        res.push((x, y - 1));
    }
    if y < cave.len() - 1 {
        res.push((x, y + 1));
    }

    res.into_iter()
}

// fn lowest_risk(memo: &mut HashMap<(usize, usize), usize>, cave: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
//     if x == 0 && y == 0 {
//         return 0;
//     }
//
//     if let Some(&c) = memo.get(&(x, y)) {
//         return c;
//     }
//
//
//     let mut risk = usize::MAX;
//
//     if x > 0 {
//         risk = risk.min(lowest_risk(memo, cave, x - 1, y));
//     }
//     if y > 0 {
//         risk = risk.min(lowest_risk(memo, cave, x, y - 1));
//     }
//
//     risk += cave[y][x] as usize;
//
//     memo.insert((x, y), risk);
//
//     risk
// }

pub fn part2(input: &mut dyn Read) -> String {
    let mut reader = BufReader::new(input);

    let mut cave = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        // println!("{}", line);

        let mut row = line.chars()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<u8>>();

        row.reserve(row.len() * 4);

        let original_len = row.len();

        // 4 copies
        for copy in 1..=4 {
            for i in 0..original_len {
                let val = row[i];
                if val >= 10 - copy {
                    row.push(1 + val - (10 - copy));
                } else {
                    row.push(val + copy);
                }
            }
        }

        cave.push(
            row
        );
    }

    cave.reserve(cave.len() * 4);

    let original_height = cave.len();
    for copy in 1..=4 {
        for i in 0..original_height {
            let mut row = cave[i].clone();
            for j in 0..row.len() {
                let val = &mut row[j];
                if *val >= 10 - copy {
                    *val = 1 + *val - (10 - copy);
                } else {
                    *val = *val + copy;
                }
            }
            cave.push(row);
        }
    }

    // for row in &cave {
    //     for val in row {
    //         print!("{}", val);
    //     }
    //     println!();
    //     // println!("{:?}", row);
    // }

    let target_x = cave[0].len() - 1;
    let target_y = cave.len() - 1;

    let mut costs: HashMap<(usize, usize), usize> = HashMap::with_capacity(cave.len() * cave[0].len());
    costs.insert((0, 0), 0);
    let mut worklist = BinaryHeap::with_capacity(cave.len() + cave[0].len());
    worklist.push((Reverse(0), (0,0)));

    while !worklist.is_empty() {
        let (Reverse(curr_cost), (curr_x, curr_y)) = worklist.pop().unwrap();

        // if (curr_x, curr_y) == (target_x, target_y) {
        //     return curr_cost.to_string();
        // }

        for (new_x, new_y) in adjacents(&cave, curr_x, curr_y) {
            let old_cost = *costs.get(&(new_x, new_y)).unwrap_or(&usize::MAX);
            let new_cost = curr_cost + cave[new_y][new_x] as usize;
            if new_cost < old_cost {
                worklist.push((Reverse(new_cost), (new_x, new_y)));
                costs.insert((new_x, new_y), new_cost);
            }
        }

    }

    costs[&(target_x, target_y)].to_string()
}



#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/15.txt");
    const REAL: &[u8] = include_bytes!("../inputs/15.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 40);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 562);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 315);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 2874);
    }
}