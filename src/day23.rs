use std::io::Read;
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
            A => 2,
            B => 4,
            C => 6,
            D => 8,
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

type Grid = [[Cell; 11]; 3];

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

// fn move_type(grid: &Grid, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Grid {
//     let mut new_grid = *grid;
//     assert!(matches!(grid[from_y][from_x], Filled(_) /*| FinalFilled(_)*/));
//     assert_eq!(grid[to_y][to_x], Empty);
//
//     new_grid[from_y][from_x] = Empty;
//     new_grid[to_y][to_x] = grid[from_y][from_x];
//
//     // if let Filled(t) = new_grid[to_y][to_x] {
//     //     if to_x == t.target_x() && from_y == 1 && to_y == 2 /* only adjust if moving top down */ {
//     //         new_grid[to_y][to_x] = FinalFilled(t);
//     //     }
//     // }
//
//     new_grid
// }

// fn neighbor_coords(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
//     let mut coords = vec![];
//     let self_type = match grid[y][x] {
//         Filled(t) => t,
//         _ => panic!("neighbor_coords called on non-filled cell")
//     };
//     if x > 0 && matches!(grid[y][x - 1], Empty) {
//         coords.push((x - 1, y));
//     }
//     if x < 12 && matches!(grid[y][x + 1], Empty) {
//         coords.push((x + 1, y));
//     }
//     // TODO if is in top slot of room, then can only move up (y-1) if it hasn't already reached its destination
//     if y == 2 && matches!(grid[y - 1][x], Empty) {
//         coords.push((x, y - 1));
//     }
//     if y == 3 && matches!(grid[y - 1][x], Empty) {
//         coords.push((x, y - 1));
//     }
//     if y == 1 && matches!(grid[y + 1][x], Empty) && self_type.target_x() == x {
//         coords.push((x, y + 1));
//     }
//     if y == 2 && matches!(grid[y + 1][x], Empty) {
//         coords.push((x, y + 1));
//     }
//     coords
// }

fn succs(grid: &Grid) -> Vec<(Grid, usize)> {
    let mut succs = vec![];

    fn possible_waiting_spot(x: usize) -> bool {
        !(x == 2 || x == 4 || x == 6 || x == 8)
    }

    // First check all dudes in the hallway: for each one check if it can move into the upper slot
    // of its destination room and the lower room
    for x in 0..11 {
        let y = 0;
        if let Filled(t) = grid[y][x] {
            let to_x = t.target_x();
            if (x+1..=to_x).all(|x| grid[y][x] == Empty) && (to_x..x).all(|x| grid[y][x] == Empty) {
                if grid[1][to_x] == Empty {
                    if grid[2][to_x] == Empty {
                        let mut new_grid = *grid;
                        new_grid[y][x] = Empty;
                        new_grid[2][to_x] = FinalFilled(t);

                        let cost = ((to_x as isize - x as isize).abs() as usize + 2) * t.cost();
                        succs.push((new_grid, cost));
                    } else {
                        let mut new_grid = *grid;
                        new_grid[y][x] = Empty;
                        new_grid[1][to_x] = FinalFilled(t);

                        let cost = ((to_x as isize - x as isize).abs() as usize + 1) * t.cost();
                        succs.push((new_grid, cost));
                    }
                }
            }
        }
    }

    // Then check all dudes that are still in their original room to move outside
    for from_x in 2..=8 {
        if let Filled(t) = grid[1][from_x] {
            // It may go left
            for to_x in (0..from_x).rev() {
                if grid[0][to_x] != Empty {
                    break;
                }
                if possible_waiting_spot(to_x) {
                    let mut new_grid = *grid;
                    new_grid[1][from_x] = Empty;
                    new_grid[0][to_x] = Filled(t);
                    let cost = (from_x - to_x + 1) * t.cost();
                    succs.push((new_grid, cost));
                }
            }
            // It may go right
            for to_x in from_x+1..11 {
                if grid[0][to_x] != Empty {
                    break;
                }
                if possible_waiting_spot(to_x) {
                    let mut new_grid = *grid;
                    new_grid[1][from_x] = Empty;
                    new_grid[0][to_x] = Filled(t);
                    let cost = (to_x - from_x + 1) * t.cost();
                    succs.push((new_grid, cost));
                }
            }
        } else if grid[1][from_x] == Empty {
            if let Filled(t) = grid[2][from_x] {
                // It may go left
                for to_x in (0..from_x).rev() {
                    if grid[0][to_x] != Empty {
                        break;
                    }
                    if possible_waiting_spot(to_x) {
                        let mut new_grid = *grid;
                        new_grid[2][from_x] = Empty;
                        new_grid[0][to_x] = Filled(t);
                        let cost = (from_x - to_x + 2) * t.cost();
                        succs.push((new_grid, cost));
                    }
                }
                // It may go right
                for to_x in from_x+1..11 {
                    if grid[0][to_x] != Empty {
                        break;
                    }
                    if possible_waiting_spot(to_x) {
                        let mut new_grid = *grid;
                        new_grid[2][from_x] = Empty;
                        new_grid[0][to_x] = Filled(t);
                        let cost = (to_x - from_x + 2) * t.cost();
                        succs.push((new_grid, cost));
                    }
                }
            }
        }
    }

    succs
}

pub fn part1(input: &mut dyn Read) -> String {
    type X = Grid;

    let mut grid = [[Wall; 11]; 3];
    for i in 0..=10 {
        grid[0][i] = Empty;
    }

    // SAMPLE:
    grid[1][2] = Filled(B); grid[1][4] = Filled(C); grid[1][6] = Filled(B); grid[1][8] = Filled(D);
    grid[2][2] = Filled(A); grid[2][4] = Filled(D); grid[2][6] = Filled(C); grid[2][8] = Filled(A);

    // REAL:
    grid[1][2] = Filled(D); grid[1][4] = Filled(D); grid[1][6] = Filled(C); grid[1][8] = Filled(C);
    grid[2][2] = Filled(B); grid[2][4] = Filled(A); grid[2][6] = Filled(B); grid[2][8] = Filled(A);

    print_grid(&grid);

    let (path, cost) = dijkstra(
        &grid,
        |grid| {
            // println!("Checking grid:");
            // print_grid(grid);

            return succs(grid);


            // let mut succs = vec![];
            // // let pre = Instant::now();
            //
            // // find all dudes that are still in their original cells, and add to succs all possible grids
            // // if they would move out to some place
            // for from_y in 1..=3 {
            //     for from_x in 1..=11 {
            //         if let Filled(t) = grid[from_y][from_x] {
            //             for (to_x, to_y) in neighbor_coords(&grid, from_x, from_y) {
            //                 // println!("{:?} -> {:?}", (from_x, from_y), (to_x, to_y));
            //                 succs.push((move_type(grid, from_x, from_y, to_x, to_y), t.cost()));
            //             }
            //         }
            //         if let FinalFilled(t) = grid[from_y][from_x] {
            //             if from_y == 2 && matches!(grid[from_y + 1][from_x], Empty) {
            //                 succs.push((move_type(grid, from_x, from_y, from_x, from_y + 1), t.cost()));
            //             }
            //         }
            //     }
            // }
            // // println!("calculating neighbors took {:?}", pre.elapsed());
            //
            // succs
        },
        |grid| {
            grid[1][2].filled_with(A) && grid[1][4].filled_with(B) && grid[1][6].filled_with(C) && grid[1][8].filled_with(D) &&
            grid[2][2].filled_with(A) && grid[2][4].filled_with(B) && grid[2][6].filled_with(C) && grid[2][8].filled_with(D)
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