use std::fmt::Debug;
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InstructionKind {
    Off,
    On,
}

use InstructionKind::*;

struct Instruction {
    opcode: InstructionKind,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut steps: Vec<Instruction> = vec![];

    for line in lines {
        let op = if &line[0..3] == "off" {
            Off
        } else {
            On
        };
        let coords = line.split(" ").last().unwrap().split(",").map(|coord| {
            let bounds = coord[2..].split("..").map(|x| {
                x.parse::<i64>().unwrap()
            }).collect::<Vec<i64>>();
            (bounds[0], bounds[1])
        }).collect::<Vec<_>>();
        steps.push(Instruction {
            opcode: op,
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    let mut grid = [[[false; 101]; 101]; 101];
    for step in steps {
        let (x, y, z) = (step.x, step.y, step.z);
        for x in x.0.max(-50)..=x.1.min(50) {
            for y in y.0.max(-50)..=y.1.min(50) {
                for z in z.0.max(-50)..=z.1.min(50) {
                    // let prev = grid[z as usize + 50][y as usize + 50][x as usize + 50];
                    match step.opcode {
                        Off => grid[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] = false,
                        On => grid[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] = true,
                    }
                }
            }
        }
    }

    // count how many trues
    let mut count = 0;
    for x in 0..101 {
        for y in 0..101 {
            for z in 0..101 {
                if grid[z][y][x] {
                    count += 1;
                }
            }
        }
    }
    count.to_string()
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cuboid {
    kind: InstructionKind,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cuboid {
    fn new(opcode: InstructionKind, x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> Cuboid {
        Cuboid {
            kind: opcode,
            x: x,
            y: y,
            z: z,
        }
    }

    fn volume(&self) -> usize {
        let (dx, dy, dz) = (self.x.1 - self.x.0 + 1, self.y.1 - self.y.0 + 1, self.z.1 - self.z.0 + 1);
        dx as usize * dy as usize * dz as usize
    }

    fn valid(&self) -> bool {
        let (dx, dy, dz) = (self.x.1 - self.x.0, self.y.1 - self.y.0, self.z.1 - self.z.0);
        dx >= 0 && dy >= 0 && dz >= 0
    }
}

impl From<Instruction> for Cuboid {
    fn from(instruction: Instruction) -> Cuboid {
        Cuboid::new(instruction.opcode, instruction.x, instruction.y, instruction.z)
    }
}

impl Debug for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} x={}..{}, y={}..{}, z={}..{})", self.kind, self.x.0, self.x.1, self.y.0, self.y.1, self.z.0, self.z.1)
    }
}

struct Grid {
    // Invariant: cuboids don't overlap
    cuboids: Vec<Cuboid>
}

fn split_up(s: &Cuboid, c: &Cuboid) -> Vec<Cuboid> {
    let mut res = vec![];

    let low_x = Cuboid::new(c.kind, (c.x.0, (s.x.0-1).min(c.x.1)), c.y, c.z);
    // println!("low_x: {:?}", low_x);
    if low_x.valid() {
        // println!("valid");
        res.push(low_x);
    }
    // let high_x = Cuboid::new(c.kind, ((s.x.1 + 1).max(c.x.0 + 1), c.x.1), c.y, c.z);
    let high_x = Cuboid::new(c.kind, ((s.x.1 + 1).max(c.x.0), c.x.1), c.y, c.z);
    // println!("high_x: {:?}", high_x);
    if high_x.valid() {
        // println!("valid");
        res.push(high_x);
    }

    // let remaining_x_bounds = (s.x.0.max(c.x.0 + 1), s.x.1.min(c.x.1 - 1));
    // let remaining_x_bounds = (low_x.x.1 + 1, high_x.x.0 - 1);
    // let remaining_x_bounds = (low_x.x.1.max(low_x.x.0) + 1, high_x.x.0.min(high_x.x.1) - 1);
    // want intersection of cube and stencil on x-axis
    let remaining_x_low = s.x.0.max(c.x.0);
    let remaining_x_high = s.x.1.min(c.x.1);
    let remaining_x_bounds = (remaining_x_low, remaining_x_high);

    let low_y = Cuboid::new(c.kind, remaining_x_bounds, (c.y.0, (s.y.0-1).min(c.y.1)), c.z);
    // println!("low_y: {:?}", low_y);
    if low_y.valid() {
        // println!("valid");
        res.push(low_y);
    }
    // let high_y = Cuboid::new(c.kind, remaining_x_bounds, ((s.y.1 + 1).max(c.y.0 + 1), c.y.1), c.z);
    let high_y = Cuboid::new(c.kind, remaining_x_bounds, ((s.y.1 + 1).max(c.y.0), c.y.1), c.z);
    // println!("high_y: {:?}", high_y);
    if high_y.valid() {
        // println!("valid");
        res.push(high_y);
    }

    // let remaining_y_bounds = (s.y.0.max(c.y.0 + 1), s.y.1.min(c.y.1 - 1));
    // let remaining_y_bounds = (low_y.y.1.max(low_y.y.0) + 1, high_y.y.0.min(high_y.y.1) - 1);

    let remaining_y_low = s.y.0.max(c.y.0);
    let remaining_y_high = s.y.1.min(c.y.1);
    let remaining_y_bounds = (remaining_y_low, remaining_y_high);

    let low_z = Cuboid::new(c.kind, remaining_x_bounds, remaining_y_bounds, (c.z.0, (s.z.0-1).min(c.z.1)));
    // println!("low_z: {:?}", low_z);
    if low_z.valid() {
        // println!("valid");
        res.push(low_z);
    }
    // let high_z = Cuboid::new(c.kind, remaining_x_bounds, remaining_y_bounds, ((s.z.1 + 1).max(c.z.0 + 1), c.z.1));
    let high_z = Cuboid::new(c.kind, remaining_x_bounds, remaining_y_bounds, ((s.z.1 + 1).max(c.z.0), c.z.1));
    // println!("high_z: {:?}", high_z);
    if high_z.valid() {
        // println!("valid");
        res.push(high_z);
    }

    // println!("Split up res length: {}", res.len());

    res
}

fn split_into_non_overlapping(overlaps: &[&Cuboid], cuboid: &Cuboid) -> Vec<Cuboid> {
    let mut res = vec![cuboid.clone()];
    for &stencil in overlaps {
        // println!("Processing stencil {:?}", stencil);
        // println!("old len: {}", res.len());
        let mut new_cuboids = vec![];
        for curr_cuboid in &res {
            // println!("Stencilling {:?} out of {:?}", stencil, curr_cuboid);
            let split_up = split_up(stencil, curr_cuboid);
            // println!("Split up into: {:?}", split_up);

            // for cub1 in &split_up {
            //     let overlaps = find_overlaps(&split_up, cub1, cub1);
            //     if overlaps.len() > 0 {
            //         println!("cub1 = {:?}, overlaps with: {:?}", cub1, overlaps);
            //         // new_cuboids.extend(split_into_non_overlapping(&overlaps, cub1));
            //         assert!(false);
            //     }
            // }
            //
            // assert!(contains(curr_cuboid, &split_up));

            new_cuboids.extend(split_up);
        }
        // println!("new len {}", new_cuboids.len());

        // for cub1 in &new_cuboids {
        //     let overlaps = find_overlaps(&new_cuboids, cub1, cub1);
        //     if overlaps.len() > 0 {
        //         println!("Found offender");
        //         println!("res: {:?}", res);
        //         println!("new_cuboids: {:?}", new_cuboids);
        //         println!("cub1 = {:?}, overlaps with: {:?}", cub1, overlaps);
        //         // new_cuboids.extend(split_into_non_overlapping(&overlaps, cub1));
        //         assert!(false);
        //     }
        // }

        res = new_cuboids;
    }

    res
}

fn contains(cub: &Cuboid, other: &[Cuboid]) -> bool {
    for other in other {
        let x_contained = cub.x.0 <= other.x.0 && cub.x.1 >= other.x.1;
        let y_contained = cub.y.0 <= other.y.0 && cub.y.1 >= other.y.1;
        let z_contained = cub.z.0 <= other.z.0 && cub.z.1 >= other.z.1;
        if !(x_contained && y_contained && z_contained) {
            return false;
        }
    }
    true
}

fn find_overlaps<'a>(others: &'a [Cuboid], cuboid: &'a Cuboid, ignore: &'a Cuboid) -> Vec<&'a Cuboid> {
    let mut overlaps = vec![];
    for other in others {
        if other == ignore {
            continue;
        }
        let x_overlap = cuboid.x.0 <= other.x.1 && cuboid.x.1 >= other.x.0;
        let y_overlap = cuboid.y.0 <= other.y.1 && cuboid.y.1 >= other.y.0;
        let z_overlap = cuboid.z.0 <= other.z.1 && cuboid.z.1 >= other.z.0;
        if x_overlap && y_overlap && z_overlap {
            overlaps.push(other);
        }
    }
    overlaps
}

impl Grid {


    fn find_overlaps(&self, cuboid: &Cuboid) -> Vec<&Cuboid> {
        let mut overlaps = vec![];
        for other in &self.cuboids {
            let x_overlap = cuboid.x.0 <= other.x.1 && cuboid.x.1 >= other.x.0;
            let y_overlap = cuboid.y.0 <= other.y.1 && cuboid.y.1 >= other.y.0;
            let z_overlap = cuboid.z.0 <= other.z.1 && cuboid.z.1 >= other.z.0;
            if x_overlap && y_overlap && z_overlap {
                overlaps.push(other);
            }
        }
        overlaps
    }

    fn add_cuboid(&mut self, cuboid: Cuboid) {
        let overlaps = self.find_overlaps(&cuboid);
        let mut new_cuboids = split_into_non_overlapping(&overlaps, &cuboid);
        self.cuboids.append(&mut new_cuboids);
    }

    fn count_on(&self) -> usize {
        let mut count = 0;
        for cuboid in &self.cuboids {
            if cuboid.kind == On {
                count += cuboid.volume();
            }
        }
        count
    }

    fn num_cuboids(&self) -> usize {
        self.cuboids.len()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            cuboids: vec![]
        }
    }
}

fn read_input(input: &mut dyn Read) -> Vec<Instruction> {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut steps: Vec<Instruction> = vec![];

    for line in lines {
        let op = if &line[0..3] == "off" {
            Off
        } else {
            On
        };
        let coords = line.split(" ").last().unwrap().split(",").map(|coord| {
            let bounds = coord[2..].split("..").map(|x| {
                x.parse::<i64>().unwrap()
            }).collect::<Vec<i64>>();
            (bounds[0], bounds[1])
        }).collect::<Vec<_>>();
        steps.push(Instruction {
            opcode: op,
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }
    steps
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut steps = read_input(input);

    // let cuboid = Cuboid::new(On, (0, 50), (0, 50), (0, 50));
    // let stencil = Cuboid::new(On, (-10, 51), (-10,115), (-10,51));
    //
    // let res = split_up(&stencil, &cuboid);
    //
    // for cub1 in &res {
    //     let overlaps = find_overlaps(&res, cub1, cub1);
    //     if overlaps.len() > 0 {
    //         println!("cub1 = {:?}, overlaps with: {:?}", cub1, overlaps);
    //         assert!(false);
    //     }
    // }
    //
    // println!("{:?}", res);

    // let cuboid = Cuboid::new(On, (1,1), (-20,20), (-30,30));
    // let stencil = Cuboid::new(Off, (1,1), (0,0), (0,0));
    //
    // let res = split_up(&stencil, &cuboid);
    // println!("{:?}", res);
    //
    // for cub1 in &res {
    //     let overlaps = find_overlaps(&res, cub1, cub1);
    //     if overlaps.len() > 0 {
    //         println!("cub1 = {:?}, overlaps with: {:?}", cub1, overlaps);
    //         assert!(false);
    //     }
    // }
    //
    //
    //
    // panic!("Not implemented");

    let mut grid = Grid::default();

    let rev_steps = steps.into_iter().rev().collect::<Vec<_>>();

    for step in rev_steps {
        let cuboid = step.into();
        // println!("num: {}", grid.num_cuboids());
        // println!("grid: {:#?}", grid.cuboids);
        // println!("Processing step {:?}", cuboid);
        grid.add_cuboid(cuboid);
    }
    // println!("num: {}", grid.num_cuboids());
    // println!("grid: {:#?}", grid.cuboids);

    grid.count_on().to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/22.txt");
    const SAMPLEP2: &[u8] = include_bytes!("samples/22.p2.txt");
    const REAL: &[u8] = include_bytes!("../inputs/22.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 590784);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 601104);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLEP2, 2758514936282235usize);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 1262883317822267usize);
    }
}