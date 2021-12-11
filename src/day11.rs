use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};

fn sim_flash(grid: &mut Vec<Vec<u8>>, did_flash: &mut Vec<Vec<bool>>, y: usize, x: usize) {
    assert!(!did_flash[y][x]);
    did_flash[y][x] = true;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                // Don't add one to self
                continue;
            }
            let y = y as isize;
            let x = x as isize;

            let n_y = (y + dy);
            let n_x = (x + dx);
            if n_y < 0 || n_y >= grid.len() as isize {
                continue;
            }
            if n_x < 0 || n_x >= grid[0].len() as isize {
                continue;
            }

            let n_y = n_y as usize;
            let n_x = n_x as usize;
            grid[n_y][n_x] += 1;
            if grid[n_y][n_x] > 9 && !did_flash[n_y][n_x] {
                sim_flash(grid, did_flash, n_y, n_x);
            }
        }
    }
}

// returns count of flashes
fn sim_step(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut flashes = 0;

    let mut did_flash = vec![vec![false; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            grid[y][x] += 1;

            if grid[y][x] > 9 && !did_flash[y][x] {
                // simulate neighbor's flash
                sim_flash(grid, &mut did_flash, y, x);
            }
        }
    }

    print_grid(grid);
    // println!();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if did_flash[y][x] {
                grid[y][x] = 0;
                flashes += 1;
            }
        }
    }


    flashes
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    print!("{}", termion::cursor::Goto(1,1));
    // print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
    let colors: [Box<dyn Display>; 5] = [
        Box::new(termion::color::Bg(termion::color::Black)),
        Box::new(termion::color::Bg(termion::color::Rgb(40,40,40))),
        Box::new(termion::color::Bg(termion::color::Rgb(80,80,80))),
        Box::new(termion::color::Bg(termion::color::Rgb(150,150,150))),
        Box::new(termion::color::Bg(termion::color::Rgb(200,200,200))),
        // Box::new(termion::color::Bg(termion::color::LightBlue)),
        // Box::new(termion::color::Bg(termion::color::Cyan)),
        // Box::new(termion::color::Bg(termion::color::LightGreen)),
        // &termion::color::Bg(termion::color::Blue),
        // &termion::color::Bg(termion::color::LightBlue),
        // &termion::color::Bg(termion::color::LightGreen),
        // &termion::color::Bg(termion::color::White),
    ];
    for row in grid {
        for &cell in row {
            if cell <= 9 {
                print!("{}  ", termion::color::Bg(termion::color::Rgb(255/10 * cell,255/10 * cell,255/10 * cell)));
                // print!("{}.", colors[(cell/2) as usize]);
                // print!("{}", cell);
            } else {
                print!("{}  ", termion::color::Bg(termion::color::Yellow));
            }
        }
        println!();
    }
    print!("{}", termion::color::Bg(termion::color::Reset));
    std::thread::sleep(std::time::Duration::from_millis(150));
}

fn grid_from_input(input: &mut dyn Read) -> Vec<Vec<u8>> {
    BufReader::new(input)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.chars().map(|c| {
                c.to_digit(10).unwrap() as u8
            }).collect()
        }).collect()
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut grid = grid_from_input(input);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += sim_step(&mut grid);
    }

    flashes.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut grid = grid_from_input(input);

    // clear grid
    print!("{}", termion::clear::All);

    // Not really used here
    let mut steps = 0;

    let max_flashes = grid.len() * grid[0].len();
    let mut flashes = 0;
    while flashes != max_flashes {
        flashes = sim_step(&mut grid);
        // let mut did_flash = vec![vec![false; grid[0].len()]; grid.len()];
        // for y in 0..grid.len() {
        //     for x in 0..grid[0].len() {
        //         // in a step every octopus gets one energy
        //         grid[y][x] += 1;
        //
        //         if grid[y][x] > 9 && !did_flash[y][x] {
        //             // simulate neighbor's flash
        //             sim_flash(&mut grid, &mut did_flash, y, x);
        //         }
        //     }
        // }
        //
        // // reset all flashes
        // for y in 0..grid.len() {
        //     for x in 0..grid[0].len() {
        //         if did_flash[y][x] {
        //             grid[y][x] = 0;
        //             flashes += 1;
        //         }
        //     }
        // }

        steps += 1;
    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/11.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 1656);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 195);
    }
}