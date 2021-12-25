use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    East,
    South,
}
use Cell::*;

fn step_south(grid: &Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, bool) {
    let mut new_grid = vec![vec![Empty; grid[0].len()]; grid.len()];
    let mut did_change = false;

    // fill with Easts
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == East {
                new_grid[y][x] = East;
            }
        }
    }
    // fill with Souths
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == South {
                let new_y = if y == grid.len() - 1 { 0 } else { y + 1 };
                if grid[new_y][x] == Empty {
                    new_grid[new_y][x] = South;
                    did_change = true;
                } else {
                    new_grid[y][x] = South;
                }
            }

        }
    }

    (new_grid, did_change)
}

fn step_east(grid: &Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, bool) {
    let mut new_grid = vec![vec![Empty; grid[0].len()]; grid.len()];
    let mut did_change = false;

    // fill with Souths
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == South {
                new_grid[y][x] = South;
            }
        }
    }
    // fill with Easts
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == East {
                let new_x = if x == grid[0].len() - 1 { 0 } else { x + 1 };
                if grid[y][new_x] == Empty {
                    new_grid[y][new_x] = East;
                    did_change = true;
                } else {
                    new_grid[y][x] = East;
                }
            }

        }
    }

    (new_grid, did_change)
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                Empty => print!("."),
                East => print!(">"),
                South => print!("v"),
            }
        }
        println!();
    }
    println!();
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut grid = BufReader::new(input).lines().map(|l| {
        let l = l.unwrap();
        l.chars().map(|c| match c {
            '.' => Empty,
            '>' => East,
            'v' => South,
            _ => panic!("invalid char"),
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // print_grid(&grid);
    // let (new_grid, _) = step_east(&grid);
    // let (new_grid, _) = step_south(&new_grid);
    // print_grid(&new_grid);

    let mut steps = 0;
    loop {
        let (new_grid, did_change_east) = step_east(&grid);
        let (new_grid, did_change_south) = step_south(&new_grid);
        steps += 1;
        if !did_change_east && !did_change_south {
            break;
        }
        grid = new_grid;
    }

    steps.to_string()
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/25.txt");
    const REAL: &[u8] = include_bytes!("../inputs/25.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 58);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 367);
    }
}