use std::io::{BufRead, BufReader, Read};

struct Image {
    explicit: Vec<Vec<bool>>,
    outside: bool,
}

impl Image {
    fn get(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return self.outside;
        }
        if x >= self.explicit.len() as isize {
            return self.outside;
        }
        if y >= self.explicit[x as usize].len() as isize {
            return self.outside;
        }
        self.explicit[y as usize][x as usize]
    }

    fn kernel_index(&self, x: isize, y: isize) -> usize {
        let mut index = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                index <<= 1;
                index += self.get(x + dx, y + dy) as usize;
            }
        }
        index
    }

    fn process(&mut self, kernel: &[bool]) {
        let new_y = self.explicit.len() + 2;
        let new_x = self.explicit[0].len() + 2;
        let mut new_image = vec![vec![false; new_x]; new_y];
        for y in 0..new_y {
            for x in 0..new_x {
                let index = self.kernel_index(x as isize - 1, y as isize - 1);
                new_image[y][x] = kernel[index as usize];
            }
        }
        // Take something that's in the infinite grid and doesn't touch the explicit image
        self.outside = kernel[self.kernel_index(-2, -2)];
        self.explicit = new_image;
    }

    fn count_ones(&self) -> Result<usize, String> {
        if self.outside {
            return Err("infinitely many ones, outside is #".to_string());
        }
        let sum = self.explicit.iter().map(|row| row.iter().filter(|&&x| x).count()).sum();
        Ok(sum)
    }

    fn print(&self) {
        for y in 0..self.explicit.len() {
            for x in 0..self.explicit[0].len() {
                print!("{}", if self.explicit[y][x] { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }
}

impl From<Vec<Vec<bool>>> for Image {
    fn from(explicit: Vec<Vec<bool>>) -> Self {
        Image {
            explicit,
            outside: false,
        }
    }
}

fn read_input(input: &mut dyn Read) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut lines = BufReader::new(input).lines();
    let image_enhancement_algorithm = lines.next().unwrap().unwrap().chars().map(|c| c == '#').collect::<Vec<_>>();
    lines.next();
    let image = lines.map(|line| line.unwrap().chars().map(|c| c == '#').collect()).collect();

    (image_enhancement_algorithm, image)
}

fn run_steps(input: &mut dyn Read, steps: usize) -> String {
    let (image_enhancement_algorithm, image) = read_input(input);
    let mut image: Image = image.into();

    // image.print();
    for _ in 0..steps {
        image.process(&image_enhancement_algorithm);
        // image.print();
    }

    image.count_ones().unwrap().to_string()
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