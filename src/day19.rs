use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Position {
    fn new(x: isize, y: isize, z: isize) -> Position {
        Position { x, y, z }
    }

    fn distance(self, other: Position) -> usize {
        (self.x - other.x).abs() as usize
            + (self.y - other.y).abs() as usize
            + (self.z - other.z).abs() as usize
    }

    // These rotations might actually all be counter-clockwise, but I'm not sure
    // Rotate around the x-axis clock-wise in 90 degree steps
    fn rotate_x(self, times: u8) -> Position {
        assert!(0 <= times && times <= 3);

        let mut new_pos = self;
        for _ in 0..times {
            new_pos = Position::new(new_pos.x, -new_pos.z, new_pos.y);
        }
        new_pos
    }

    // Rotate around the y-axis clock-wise in 90 degree steps
    fn rotate_y(self, times: u8) -> Position {
        assert!(0 <= times && times <= 3);

        let mut new_pos = self;
        for _ in 0..times {
            new_pos = Position::new(new_pos.z, new_pos.y, -new_pos.x);
        }
        new_pos
    }

    // Rotate around the z-axis clock-wise in 90 degree steps
    fn rotate_z(self, times: u8) -> Position {
        assert!(0 <= times && times <= 3);

        let mut new_pos = self;
        for _ in 0..times {
            new_pos = Position::new(-new_pos.y, new_pos.x, new_pos.z);
        }
        new_pos
    }

    fn get_possible_facings(self) -> [Position; 6] {
        [
            self.rotate_z(0),
            self.rotate_z(1),
            self.rotate_z(2),
            self.rotate_z(3),
            self.rotate_y(1),
            self.rotate_y(3),
        ]
    }

    fn get_possible_orientations(self) -> [Position; 24] {
        let mut orientations = [self; 24];
        let facings = self.get_possible_facings();
        for i in 0..4 {
            for j in 0..6 {
                orientations[i * 6 + j] = facings[j].rotate_x(i as u8);
            }
        }
        orientations
    }
}

fn normalize_to_ith(scanner: &[Position], i: usize) -> HashSet<Position> {
    let ith = scanner[i];
    let mut new_scanner = HashSet::new();
    for pos in scanner {
        new_scanner.insert(Position::new(pos.x - ith.x, pos.y - ith.y, pos.z - ith.z));
    }
    new_scanner
}

fn normalize_to_first(scanner: &[Position]) -> HashSet<Position> {
    normalize_to_ith(scanner, 0)
}

fn normalize_all(scanner: &[Position]) -> Vec<HashSet<Position>> {
    let mut all = vec![];
    for i in 0..scanner.len() {
        all.push(normalize_to_ith(scanner, i));
    }
    all
}

fn num_overlapping(scanner1: &HashSet<Position>, scanner2_orientations_normalizations: &[(Position, HashSet<Position>)]) -> (usize, Option<(Position, Vec<Position>)>) {
    // let scanner2_orientations = get_all_rotations(scanner2);
    let mut max_same = 0;
    let mut max_content = None;
    // let mut max_relative_pos: Option<Position> = None;
    // let mut max_orientation: Option<Vec<Position>> = None;
    for &scanner1_offset in scanner1 {
        let scanner1_set = scanner1.into_iter().map(|&pos| pos - scanner1_offset).collect::<HashSet<_>>();
        // let mut scanner1_set = scanner1.iter().collect::<HashSet<_>>();

        for (scanner2_offset, scanner2_set) in scanner2_orientations_normalizations {
            // let scanner2_offset = orientation[i];
            // let scanner2 = normalize_to_ith(&orientation, i);
            // let mut set = scanner2.into_iter().collect::<HashSet<_>>();
            let num_same = scanner1_set.intersection(&scanner2_set).count();

            // let mut num_same = 0;
            // for &pos in scanner1 {
            //     if scanner2_set.contains(&(pos - scanner1_offset)) {
            //         num_same += 1;
            //     }
            // }
            if num_same > max_same {
                if num_same >= 12 {
                    let relative_pos = scanner1_offset - *scanner2_offset;
                    let orientation_relative_to_first = scanner2_set.into_iter().map(|&pos| pos + relative_pos + *scanner2_offset).collect::<Vec<_>>();
                    max_content = Some((relative_pos, orientation_relative_to_first));
                }
                max_same = num_same;
            }
        }

        // for orientation in scanner2_orientations.clone() {
        //     // let orientation = normalize_to_first(&orientation);
        //     for i in 0..orientation.len() {
        //         let scanner2_offset = orientation[i];
        //         let scanner2 = normalize_to_ith(&orientation, i);
        //         let mut set = scanner2.into_iter().collect::<HashSet<_>>();
        //         let mut num_same = 0;
        //         for pos in scanner1_set.intersection(&set) {
        //             num_same += 1;
        //
        //         }
        //         if num_same > max_same {
        //             if num_same >= 12 {
        //                 let relative_pos = scanner1_offset - scanner2_offset;
        //                 let orientation_relative_to_first = orientation.clone().into_iter().map(|pos| pos + relative_pos).collect::<Vec<_>>();
        //                 max_content = Some((relative_pos, orientation_relative_to_first));
        //             }
        //             max_same = num_same;
        //         }
        //
        //     }
        //     // for normalized_orientation in normalize_all(&orientation) {
        //     //     let mut set = normalized_orientation.into_iter().collect::<HashSet<_>>();
        //     //     let mut num_same = scanner1_set.intersection(&set).count();
        //     //     if num_same > max_same {
        //     //         max_same = num_same;
        //     //     }
        //     // }
        // }
    }

    (max_same, max_content)
}

// Returns all possible combinations of rotations of all positions
fn get_all_rotations(scanner: &[Position]) -> [Vec<Position>; 24] {
    let mut rotations = [0; 24].map(|_| Vec::new());
    for pos in scanner {
        let orientations = pos.get_possible_orientations();
        // must be 24 different positions
        assert!(orientations.iter().collect::<HashSet<_>>().len() == 24);
        for i in 0..24 {
            rotations[i].push(orientations[i]);
        }
    }
    // for z_rot in 0..4 {
    //     for y_rot in 0..4 {
    //         for x_rot in 0..4 {
    //             let mut new_scanner = scanner.clone();
    //             for pos in &mut new_scanner {
    //                 *pos = pos.rotate_x(x_rot);
    //             }
    //             for pos in &mut new_scanner {
    //                 *pos = pos.rotate_y(y_rot);
    //             }
    //             for pos in &mut new_scanner {
    //                 *pos = pos.rotate_z(z_rot);
    //             }
    //             rotations[(z_rot * 4 + y_rot) * 4 + x_rot].extend(new_scanner);
    //         }
    //     }
    // }

    rotations
}

fn read_input(input: &mut dyn Read) -> Vec<Vec<Position>> {
    let mut buf = BufReader::new(input);
    let mut scanners = vec![];
    let mut curr_beacons = vec![];
    let mut lines = buf.lines();
    // Skip the first line -- scanner 0 --
    lines.next();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line == "" {
            scanners.push(curr_beacons);
            curr_beacons = vec![];
            // Read -- scanner X --- stuff
            lines.next();
        } else {
            // println!("line: {}", line);
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse::<isize>().unwrap();
            let y = parts.next().unwrap().parse::<isize>().unwrap();
            let z = parts.next().unwrap().parse::<isize>().unwrap();
            curr_beacons.push(Position::new(x, y, z));
        }
    }
    if curr_beacons.len() > 0 {
        scanners.push(curr_beacons);
    }

    scanners
}

pub fn part1(input: &mut dyn Read) -> String {
    let scanners = read_input(input);
    // println!("{:#?}", scanners);

    let scanner0 = scanners[0].clone();
    // println!("{:#?}", scanner0);
    let mut all_rotations = get_all_rotations(&scanner0);
    // println!("{:#?}", all_rotations);

    // let (num, relative_pos) = num_overlapping(&scanners[0], &scanners[1]);
    // println!("Overlapping 0 and 1: {}, {:#?}", num, relative_pos);
    //
    // let (rel, normalized_1) = relative_pos.unwrap();
    // println!("Original 1: {:#?}", scanners[1]);
    // println!("Normalizing 1: {:#?}", normalized_1);
    //
    // // println!("scanner 4: {:#?}", scanners[4]);
    // let (num, relative_pos) = num_overlapping(&scanners[1], &scanners[4]);
    // println!("Overlapping 1 and 4: {}, {:#?}", num, relative_pos);

    let mut scanners_to_normalize = scanners[1..].iter().map(|scanner| {
        let mut res = vec![];
        for orientation in get_all_rotations(scanner) {
            for i in 0..orientation.len() {
                let offset = orientation[i];
                let normalized_orientation = normalize_to_ith(&orientation, i);
                res.push((offset, normalized_orientation));
            }
        }
        res
    }).collect::<Vec<_>>();
    let mut normalized_scanners = scanners[0..1].to_vec();
    let mut normalized_scanner_final = scanners[0].clone().into_iter().collect::<HashSet<_>>();

    'outer: while scanners_to_normalize.len() > 0 {
        println!("normalize remaining: {}", scanners_to_normalize.len());
        println!("normalized scanners: {}", normalized_scanners.len());
        for (i, scanner_to_normalize) in scanners_to_normalize.iter().enumerate() {
            // let normalized_scanner = normalized_scanner_final.clone().into_iter().collect::<Vec<_>>();
            // Check if scanners overlap
            let (num, relative_pos) = num_overlapping(&normalized_scanner_final, scanner_to_normalize);
            println!("overlapping: {}", num);
            if let Some((new_offset, new_normalized)) = relative_pos {
                normalized_scanner_final.extend(new_normalized);
                // normalized_scanners.push(new_normalized);
                scanners_to_normalize.remove(i);
                continue 'outer;
            }
            // for normalized_scanner in &normalized_scanners {
            //     // Check if scanners overlap
            //     let (num, relative_pos) = num_overlapping(normalized_scanner, scanner_to_normalize);
            //     println!("overlapping: {}", num);
            //     if let Some((_, new_normalized)) = relative_pos {
            //         normalized_scanner_final.extend(new_normalized.clone());
            //         normalized_scanners.push(new_normalized);
            //         scanners_to_normalize.remove(i);
            //         continue 'outer;
            //     }
            //
            // }
        }

    }

    // let (num, relative_pos) = num_overlapping(&scanners[0], &scanners[1]);
    // println!("Overlapping 0 and 1: {}, {:#?}", num, relative_pos);

    normalized_scanner_final.len().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let scanners = read_input(input);
    // println!("{:#?}", scanners);

    let scanner0 = scanners[0].clone();
    // println!("{:#?}", scanner0);
    let mut all_rotations = get_all_rotations(&scanner0);
    // println!("{:#?}", all_rotations);

    // let (num, relative_pos) = num_overlapping(&scanners[0], &scanners[1]);
    // println!("Overlapping 0 and 1: {}, {:#?}", num, relative_pos);
    //
    // let (rel, normalized_1) = relative_pos.unwrap();
    // println!("Original 1: {:#?}", scanners[1]);
    // println!("Normalizing 1: {:#?}", normalized_1);
    //
    // // println!("scanner 4: {:#?}", scanners[4]);
    // let (num, relative_pos) = num_overlapping(&scanners[1], &scanners[4]);
    // println!("Overlapping 1 and 4: {}, {:#?}", num, relative_pos);

    let mut scanners_to_normalize = scanners[1..].iter().map(|scanner| {
        let mut res = vec![];
        for orientation in get_all_rotations(scanner) {
            for i in 0..orientation.len() {
                let offset = orientation[i];
                let normalized_orientation = normalize_to_ith(&orientation, i);
                res.push((offset, normalized_orientation));
            }
        }
        res
    }).collect::<Vec<_>>();
    let mut normalized_scanners = scanners[0..1].to_vec();
    let mut normalized_scanner_final = scanners[0].clone().into_iter().collect::<HashSet<_>>();

    let mut offsets = vec![Position::new(0, 0, 0)];

    'outer: while scanners_to_normalize.len() > 0 {
        println!("normalize remaining: {}", scanners_to_normalize.len());
        println!("normalized scanners: {}", normalized_scanners.len());
        for (i, scanner_to_normalize) in scanners_to_normalize.iter().enumerate() {
            // let normalized_scanner = normalized_scanner_final.clone().into_iter().collect::<Vec<_>>();
            // Check if scanners overlap
            let (num, relative_pos) = num_overlapping(&normalized_scanner_final, scanner_to_normalize);
            println!("overlapping: {}", num);
            if let Some((new_offset, new_normalized)) = relative_pos {
                normalized_scanner_final.extend(new_normalized);
                offsets.push(new_offset);
                // normalized_scanners.push(new_normalized);
                scanners_to_normalize.remove(i);
                continue 'outer;
            }
            // for normalized_scanner in &normalized_scanners {
            //     // Check if scanners overlap
            //     let (num, relative_pos) = num_overlapping(normalized_scanner, scanner_to_normalize);
            //     println!("overlapping: {}", num);
            //     if let Some((new_offset, new_normalized)) = relative_pos {
            //         normalized_scanner_final.extend(new_normalized.clone());
            //         normalized_scanners.push(new_normalized);
            //         scanners_to_normalize.remove(i);
            //         offsets.push(new_offset);
            //         continue 'outer;
            //     }
            //
            // }
        }

    }

    println!("{:#?}", offsets);

    let mut max_dist = 0;
    for i in 0..offsets.len() {
        for j in i+1..offsets.len() {
            let dist = offsets[i].distance(offsets[j]);
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    // let (num, relative_pos) = num_overlapping(&scanners[0], &scanners[1]);
    // println!("Overlapping 0 and 1: {}, {:#?}", num, relative_pos);

    max_dist.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/19.txt");
    // const SAMPLE2: &[u8] = include_bytes!("samples/19.2.txt");
    const REAL: &[u8] = include_bytes!("../inputs/19.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 79);
    }

    // #[test]
    // fn sample2_part1() {
    //     test_implementation(part1, SAMPLE2, 4140);
    // }

    // Took 72 seconds...
    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 467);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 3621);
    }

    // Took 68 seconds...
    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 12226);
    }
}