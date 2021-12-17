use std::io::Read;

type Position = (isize, isize);
type Velocity = (isize, isize);
// lower-left (x, y) to upper-right (x, y)
type Area = (Position, Position);

fn area_from_input(input: &mut dyn Read) -> Area {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let parts = buf.split(" ").collect::<Vec<_>>();
    let x_range = &parts[2][2..(parts[2].len() - 1)];
    let y_range = &parts[3][2..];

    let x_range = x_range.split("..").map(|val| val.parse().unwrap()).collect::<Vec<_>>();
    let y_range = y_range.split("..").map(|val| val.parse().unwrap()).collect::<Vec<_>>();

    // ((x_range[0], x_range[1]), (y_range[0], y_range[1]))
    ((x_range[0], y_range[0]), (x_range[1], y_range[1]))
}

fn do_step(((pos_x, pos_y), (vel_x, vel_y)): (Position, Velocity)) -> (Position, Velocity) {
    let new_vel_x = if vel_x > 0 {
        vel_x - 1
    } else {
        assert_eq!(vel_x, 0);
        0
    };
    (
        ((pos_x + vel_x), (pos_y + vel_y)),
        (new_vel_x, vel_y - 1)
    )
}

// None if not hit yet,
// Some(false) if not possible anymore
// Some(true) if currently inside
fn did_hit(((pos_x, pos_y), (vel_x, vel_y)): (Position, Velocity), (target_low, target_high): Area) -> Option<bool> {
    if pos_x > target_high.0 {
        // Assumption: vel_x >= 0
        return Some(false);
    }
    if pos_y < target_low.1 && vel_y <= 0 {
        return Some(false);
    }
    // if it's within
    if pos_x >= target_low.0 && pos_x <= target_high.0 && pos_y >= target_low.1 && pos_y <= target_high.1 {
        // println!("{:?} is within {:?}", (pos_x, pos_y), (target_low, target_high));
        return Some(true);
    }

    None
}

// Returns None if impossible, Some(y) if it's possible, and y is the highest one reached
fn simulate_throw(initial_velocity: Velocity, target_area: Area) -> Option<isize> {
    let mut state = ((0, 0), initial_velocity);
    let mut steps = 0;
    let mut max_y = 0;
    loop {
        // println!("{:?}", state);
        match did_hit(state, target_area) {
            Some(true) => {
                return Some(max_y);
            }
            Some(false) => {
                return None;
            }
            None => {
                state = do_step(state);
                if state.0.1 > max_y {
                    max_y = state.0.1;
                }
                steps += 1;
            }
        }
    }
}

pub fn part1(input: &mut dyn Read) -> String {
    let target_area = area_from_input(input);

    // println!("{:?}", target_area);

    let mut max_y = 0;
    for y in -300..300 {
        for x in 0..300 {
            let velocity = (x, y);
            if let Some(y) = simulate_throw(velocity, target_area) {
                // println!("found new hitting throw: {:?}, max: {}", velocity, y);
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }

    max_y.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let target_area = area_from_input(input);

    // println!("{:?}", target_area);

    let mut num_hits = 0;
    for y in -300..300 {
        for x in 0..=target_area.1.0 {
            let velocity = (x, y);
            if let Some(_) = simulate_throw(velocity, target_area) {
                // println!("found new hitting throw: {:?}, max: {}", velocity, y);
                num_hits += 1;
            }
        }
    }

    num_hits.to_string()
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/17.txt");
    const REAL: &[u8] = include_bytes!("../inputs/17.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 45);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 5778);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 112);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 2576);
    }
}