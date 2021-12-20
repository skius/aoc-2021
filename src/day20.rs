use std::io::{BufRead, BufReader, Read};

fn read_input(input: &mut dyn Read) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut lines = BufReader::new(input).lines();
    let image_enhancement_algorithm = lines.next().unwrap().unwrap().chars().map(|c| c == '#').collect::<Vec<_>>();
    lines.next();
    let image = lines.map(|line| line.unwrap().chars().map(|c| c == '#').collect()).collect();

    (image_enhancement_algorithm, image)
}

fn get_or_default(image: &Vec<Vec<bool>>, x: isize, y: isize, default: u16) -> u16 {
    if x < 0 || x >= image[0].len() as isize {
        return default;
    }
    if y < 0 || y >= image.len() as isize {
        return default;
    }

    image[y as usize][x as usize] as u16
}

fn kernel_index(image: &Vec<Vec<bool>>, x: isize, y: isize, default: u16) -> u16 {
    let mut index = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            index <<= 1;
            index += get_or_default(image, x + dx, y + dy, default);
        }
    }
    // index <<= 1;
    // index += get_or_zero(image, x - 1, y - 1);
    // index <<= 1;
    // index += get_or_zero(image, x, y - 1);
    // index <<= 1;
    // index += get_or_zero(image, x + 1, y - 1);
    // index <<= 1;
    // index += get_or_zero(image, x - 1, y);
    // index <<= 1;
    // index += get_or_zero(image, x, y);
    // index <<= 1;
    // index += get_or_zero(image, x + 1, y);
    // index <<= 1;
    // index += get_or_zero(image, x - 1, y + 1);
    // index <<= 1;
    // index += get_or_zero(image, x, y + 1);
    // index <<= 1;
    // index += get_or_zero(image, x + 1, y + 1);

    index
}

fn step(image: &Vec<Vec<bool>>, image_enhancement_algorithm: &[bool], default: u16) -> Vec<Vec<bool>> {
    let new_y = image.len() + 2;
    let new_x = image[0].len() + 2;
    let mut new_image = vec![vec![false; new_x]; new_y];
    for y in 0..new_y {
        for x in 0..new_x {
            let index = kernel_index(&image, x as isize - 1, y as isize - 1, default);
            new_image[y][x] = image_enhancement_algorithm[index as usize];
        }
    }
    new_image
}

fn print_image(image: &Vec<Vec<bool>>) {
    for y in 0..image.len() {
        for x in 0..image[0].len() {
            print!("{}", if image[y][x] { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

fn run_steps(input: &mut dyn Read, steps: usize) -> String {
    let (image_enhancement_algorithm, mut image) = read_input(input);
    // print_image(&image);
    let does_flicker = image_enhancement_algorithm[0];
    let mut default = 0;
    for _ in 0..steps {
        image = step(&image, &image_enhancement_algorithm, default);
        default = if does_flicker { 1 - default } else { 0 };
        // print_image(&image);
    }


    image.iter().map(|row| row.iter().filter(|&&b| b).count()).sum::<usize>().to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    run_steps(input, 2)
}

pub fn part2(input: &mut dyn Read) -> String {
    run_steps(input, 50)
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/20.txt");
    const REAL: &[u8] = include_bytes!("../inputs/20.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 35);
    }

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