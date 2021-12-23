use std::io::Read;
use std::mem::size_of;
use std::time::Instant;
use pathfinding::prelude::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    A,
    B,
    C,
    D
}
use Type::*;

impl Type {
    fn cost(&self) -> usize {
        match *self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn target_x(&self) -> usize {
        match *self {
            A => 3,
            B => 5,
            C => 7,
            D => 9,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Filled(Type),
    FinalFilled(Type),
}
use Cell::*;

impl Cell {
    fn filled_with(&self, t: Type) -> bool {
        match *self {
            Filled(t2) => t == t2,
            FinalFilled(t2) => t == t2,
            _ => false,
        }
    }
}

type Grid = [[Cell; 13]; 5];

fn print_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            match cell {
                Empty => print!("."),
                Wall => print!("#"),
                Filled(A) => print!("A"),
                Filled(B) => print!("B"),
                Filled(C) => print!("C"),
                Filled(D) => print!("D"),
                FinalFilled(A) => print!("a"),
                FinalFilled(B) => print!("b"),
                FinalFilled(C) => print!("c"),
                FinalFilled(D) => print!("d"),
            }
        }
        println!();
    }
    println!();
}

fn move_type(grid: &Grid, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Grid {
    let mut new_grid = *grid;
    assert!(matches!(grid[from_y][from_x], Filled(_) | FinalFilled(_)));
    assert_eq!(grid[to_y][to_x], Empty);

    new_grid[from_y][from_x] = Empty;
    new_grid[to_y][to_x] = grid[from_y][from_x];

    if let Filled(t) = new_grid[to_y][to_x] {
        if to_x == t.target_x() && from_y == 1 && to_y == 2 /* only adjust if moving top down */ {
            new_grid[to_y][to_x] = FinalFilled(t);
        }
    }

    new_grid
}

fn neighbor_coords(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    let self_type = match grid[y][x] {
        Filled(t) => t,
        _ => panic!("neighbor_coords called on non-filled cell")
    };
    if x > 0 && matches!(grid[y][x - 1], Empty) {
        coords.push((x - 1, y));
    }
    if x < 12 && matches!(grid[y][x + 1], Empty) {
        coords.push((x + 1, y));
    }
    // TODO if is in top slot of room, then can only move up (y-1) if it hasn't already reached its destination
    if y == 2 && matches!(grid[y - 1][x], Empty) {
        coords.push((x, y - 1));
    }
    if y == 3 && matches!(grid[y - 1][x], Empty) {
        coords.push((x, y - 1));
    }
    if y == 1 && matches!(grid[y + 1][x], Empty) && self_type.target_x() == x {
        coords.push((x, y + 1));
    }
    if y == 2 && matches!(grid[y + 1][x], Empty) {
        coords.push((x, y + 1));
    }
    coords
}

pub fn part1(input: &mut dyn Read) -> String {

    let mut grid = [[Wall; 13]; 5];
    for i in 1..=11 {
        grid[1][i] = Empty;
    }

    // SAMPLE:
    // grid[2][3] = Filled(B); grid[2][5] = Filled(C); grid[2][7] = Filled(B); grid[2][9] = Filled(D);
    // grid[3][3] = Filled(A); grid[3][5] = Filled(D); grid[3][7] = Filled(C); grid[3][9] = Filled(A);

    // REAL:
    grid[2][3] = Filled(D); grid[2][5] = Filled(D); grid[2][7] = Filled(C); grid[2][9] = Filled(C);
    grid[3][3] = Filled(B); grid[3][5] = Filled(A); grid[3][7] = Filled(B); grid[3][9] = Filled(A);

    print_grid(&grid);

    let (path, cost) = dijkstra(
        &grid,
        |grid| {
            // println!("Checking grid:");
            // print_grid(grid);

            let mut succs = vec![];
            // let pre = Instant::now();

            // find all dudes that are still in their original cells, and add to succs all possible grids
            // if they would move out to some place
            for from_y in 1..=3 {
                for from_x in 1..=11 {
                    if let Filled(t) = grid[from_y][from_x] {
                        for (to_x, to_y) in neighbor_coords(&grid, from_x, from_y) {
                            // println!("{:?} -> {:?}", (from_x, from_y), (to_x, to_y));
                            succs.push((move_type(grid, from_x, from_y, to_x, to_y), t.cost()));
                        }
                    }
                    if let FinalFilled(t) = grid[from_y][from_x] {
                        if from_y == 2 && matches!(grid[from_y + 1][from_x], Empty) {
                            succs.push((move_type(grid, from_x, from_y, from_x, from_y + 1), t.cost()));
                        }
                    }
                }
            }
            // println!("calculating neighbors took {:?}", pre.elapsed());

            succs
        },
        |grid| {
            grid[2][3].filled_with(A) && grid[2][5].filled_with(B) && grid[2][7].filled_with(C) && grid[2][9].filled_with(D) &&
            grid[3][3].filled_with(A) && grid[3][5].filled_with(B) && grid[3][7].filled_with(C) && grid[3][9].filled_with(D)
        },
    ).unwrap();

    for grid in &path {
        print_grid(grid);
    }

    cost.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/23.txt");
    const REAL: &[u8] = include_bytes!("../inputs/23.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 35);
    }

    // 17109 too high for my input
    // 16051 too low
    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 4917);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 3351);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 16389);
    }
}