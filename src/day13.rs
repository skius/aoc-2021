use std::io::Read;
use std::{thread, time};

fn process_fold(fold: (u8, usize), grid: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // let coord = if fold.0 == 0 {
    //     |(x, y)| x
    // } else {
    //     |(x, y)| y
    // };

    // println!("Processing fold: {},{}", fold.0, fold.1);

    let mut result = vec![];

    for &(x, y) in grid {
        if fold.0 == 0 {
            // flipping left at fold.1
            if x < fold.1 {
                result.push((x, y));
            } else {
                result.push((2 * fold.1 - x, y));
            }
        } else {
            // flipping up at fold.1
            if y < fold.1 {
                result.push((x, y));
            } else {
                result.push((x, 2 * fold.1 - y));
            }
        }
    }

    result
}

fn read_input(input: &mut dyn Read) -> (Vec<(usize, usize)>, Vec<(u8, usize)>) {
    let mut res = String::new();
    input.read_to_string(&mut res);
    let mut res = res.trim().to_string();

    let res = res.replace("\r", "");
    let parts = res.split("\n\n").collect::<Vec<_>>();
    let grid = parts[0].split("\n").map(|line| {
        // println!("line: {}", line);
        let coords = line.split(",").map(|coord| coord.parse().unwrap()).collect::<Vec<_>>();
        (coords[0], coords[1])
    }).collect::<Vec<(usize, usize)>>();
    let folds = parts[1].split("\n").map(|line| {
        // println!("line: {}", line);
        let parts = line.split(" ").skip(2).next().unwrap().split("=").collect::<Vec<_>>();
        let first = if parts[0] == "x" {
            0
        } else {
            1
        };

        let second = parts[1].parse().unwrap();
        (first, second)
    }).collect::<Vec<(u8, usize)>>();
    // (0, num) means fold at x = num, (1, num) means fold at y = num

    (grid, folds)
}

pub fn part1(input: &mut dyn Read) -> String {
    let (grid, folds) = read_input(input);
    // (0, num) means fold at x = num, (1, num) means fold at y = num

    let mut res = process_fold(folds[0], &grid);

    res.sort();
    res.dedup();
    res.len().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let (grid, folds) = read_input(input);

    // let mut res = process_fold(folds[0], &grid);
    //
    // res.sort();
    // res.dedup();

    let mut grid = grid;
    for fold in folds {
        grid = process_fold(fold, &grid);
        grid.sort();
        grid.dedup();
    }

    print_grid(&grid);

    "".to_string()

}

fn print_grid(grid: &Vec<(usize, usize)>) {
    let n_x = grid.iter().max_by_key(|&(x, _)| x).unwrap().0 + 1;
    let n_y = grid.iter().max_by_key(|&(_, y)| y).unwrap().1 + 1;
    let mut hard_coded_grid = vec![vec![" ".to_string(); n_x]; n_y];

    for &(x, y) in grid {
        hard_coded_grid[y][x] = format!("{} {}", termion::color::Bg(termion::color::White), termion::color::Bg(termion::color::Reset));
    }

    for row in hard_coded_grid {
        println!("{}", row.into_iter().collect::<String>());
        // thread::sleep(time::Duration::from_millis(100));
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/13.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 17);
    }

    // Can't really test this (don't want to) because it's visual
    // #[test]
    // fn sample_part2() {
    //     test_implementation(part2, SAMPLE, 3509);
    // }
}