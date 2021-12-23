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

type Grid<const Y: usize> = [[Cell; 11]; Y];

fn print_grid<const Y: usize>(grid: &Grid<Y>) {
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

fn succs<const Y: usize>(grid: &Grid<Y>) -> Vec<(Grid<Y>, usize)> {
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
                let mut dest_y = 1;
                while dest_y < Y && grid[dest_y][to_x] == Empty {
                    dest_y += 1;
                }
                dest_y -= 1;
                if dest_y == 0 {
                    // no free slots
                    continue;
                }
                // Now dest_y points to the lowest free slot
                // (additionally we can now check that none of the preoccupied slots contain cells that do not belong there
                if (dest_y..Y).any(|y| {
                    if let Filled(t) = grid[y][to_x] {
                        if t.target_x() != to_x {
                            return true;
                        }
                    }
                    false
                }) {
                    continue;
                }

                let mut new_grid = *grid;
                new_grid[y][x] = Empty;
                new_grid[dest_y][to_x] = FinalFilled(t);

                let cost = ((to_x as isize - x as isize).abs() as usize + dest_y) * t.cost();
                succs.push((new_grid, cost));
            }
        }
    }

    // Then check all dudes that are still in their original room to move outside
    for from_x in 2..=8 {
        let mut from_y = 1;
        while from_y < Y  && grid[from_y][from_x] == Empty {
            from_y += 1;
        }
        // now from_y points to the highest used slot
        if from_y == Y  {
            // no dude here
            continue;
        }
        if let Filled(t) = grid[from_y][from_x] {
            // It may go left
            for to_x in (0..from_x).rev() {
                if grid[0][to_x] != Empty {
                    break;
                }
                if possible_waiting_spot(to_x) {
                    let mut new_grid = *grid;
                    new_grid[from_y][from_x] = Empty;
                    new_grid[0][to_x] = Filled(t);
                    let cost = (from_x - to_x + from_y) * t.cost();
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
                    new_grid[from_y][from_x] = Empty;
                    new_grid[0][to_x] = Filled(t);
                    let cost = (to_x - from_x + from_y) * t.cost();
                    succs.push((new_grid, cost));
                }
            }
        }
    }

    succs
}

fn solve<const Y: usize>(part2: bool) -> String {
    // println!("Size of grid: {}", std::mem::size_of::<Grid<Y>>());

    let mut grid = [[Wall; 11]; Y ];
    for i in 0..=10 {
        grid[0][i] = Empty;
    }

    if !part2 {
        // SAMPLE:
        // grid[1][2] = Filled(B); grid[1][4] = Filled(C); grid[1][6] = Filled(B); grid[1][8] = Filled(D);
        // grid[2][2] = Filled(A); grid[2][4] = Filled(D); grid[2][6] = Filled(C); grid[2][8] = Filled(A);

        // REAL:
        grid[1][2] = Filled(D); grid[1][4] = Filled(D); grid[1][6] = Filled(C); grid[1][8] = Filled(C);
        grid[2][2] = Filled(B); grid[2][4] = Filled(A); grid[2][6] = Filled(B); grid[2][8] = Filled(A);
    } else {
        // SAMPLE:
        // grid[1][2] = Filled(B); grid[1][4] = Filled(C); grid[1][6] = Filled(B); grid[1][8] = Filled(D);
        // grid[2][2] = Filled(D); grid[2][4] = Filled(C); grid[2][6] = Filled(B); grid[2][8] = Filled(A);
        // grid[3][2] = Filled(D); grid[3][4] = Filled(B); grid[3][6] = Filled(A); grid[3][8] = Filled(C);
        // grid[4][2] = Filled(A); grid[4][4] = Filled(D); grid[4][6] = Filled(C); grid[4][8] = Filled(A);

        // REAL:
        grid[1][2] = Filled(D); grid[1][4] = Filled(D); grid[1][6] = Filled(C); grid[1][8] = Filled(C);
        grid[2][2] = Filled(D); grid[2][4] = Filled(C); grid[2][6] = Filled(B); grid[2][8] = Filled(A);
        grid[3][2] = Filled(D); grid[3][4] = Filled(B); grid[3][6] = Filled(A); grid[3][8] = Filled(C);
        grid[4][2] = Filled(B); grid[4][4] = Filled(A); grid[4][6] = Filled(B); grid[4][8] = Filled(A);
    }



    // print_grid(&grid);

    let (path, cost) = dijkstra(
        &grid,
        |grid| {
            // println!("Checking grid:");
            // print_grid(grid);

            return succs(grid);
        },
        |grid| {
            for y in 1..Y {
                if !grid[y][A.target_x()].filled_with(A) {
                    return false;
                }
                if !grid[y][B.target_x()].filled_with(B) {
                    return false;
                }
                if !grid[y][C.target_x()].filled_with(C) {
                    return false;
                }
                if !grid[y][D.target_x()].filled_with(D) {
                    return false;
                }
            }

            true
        },
    ).unwrap();

    // for grid in &path {
    //     print_grid(grid);
    // }

    cost.to_string()
}



pub fn part1(input: &mut dyn Read) -> String {
    solve::<3>(false)
}

pub fn part2(input: &mut dyn Read) -> String {
    solve::<5>(true)

}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/23.txt");
    const REAL: &[u8] = include_bytes!("../inputs/23.txt");

    // #[test]
    // fn sample_part1() {
    //     test_implementation(part1, SAMPLE, 12521);
    // }

    // 17109 too high for my input
    // 16051 too low
    // 16059 correct
    // #[test]
    // fn real_part1() {
    //     test_implementation(part1, REAL, 16059);
    // }

    // 44169 correct
    // #[test]
    // fn sample_part2() {
    //     test_implementation(part2, SAMPLE, 44169);
    // }

    // 43117 correct
    // #[test]
    // fn real_part2() {
    //     test_implementation(part2, REAL, 43117);
    // }
}