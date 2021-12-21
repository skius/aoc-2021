use std::char::MAX;
use std::io::Read;
use num_bigint::BigUint;
use rustc_hash::FxHashMap;

trait Die: Default {
    fn roll(&mut self) -> usize;
    fn count_rolled(&self) -> usize;
}

struct DD100 {
    last: usize,
    roll_count: usize,
}

impl Default for DD100 {
    fn default() -> Self {
        DD100 {
            // last = 0, because we want to start with a 1
            last: 0,
            roll_count: 0,
        }
    }
}

impl Die for DD100 {
    fn roll(&mut self) -> usize {
        self.roll_count += 1;
        self.last = (self.last % 100) + 1;
        self.last
    }

    fn count_rolled(&self) -> usize {
        self.roll_count
    }
}

const MAX_POS: usize = 10;

// pos is 1-indexed position. to make maths easier, we convert to 0 index and then back to 1 index
fn update_pos(pos: usize, roll: usize) -> usize {
    (pos - 1 + roll) % MAX_POS + 1
}

fn do_turn<D: Die>(die: &mut D, mut pos: usize) -> usize {
    let roll = die.roll() + die.roll() + die.roll();
    update_pos(pos, roll)
}

const MAX_SCORE_1: usize = 1000;

fn do_game<D: Die>(p1: usize, p2: usize) -> usize {
    let mut die = D::default();
    let mut pos1 = p1;
    let mut pos2 = p2;
    let mut score1 = 0;
    let mut score2 = 0;
    loop {
        pos1 = do_turn(&mut die, pos1);
        score1 += pos1;
        if score1 >= MAX_SCORE_1 {
            return score2 * die.count_rolled();
        }
        pos2 = do_turn(&mut die, pos2);
        score2 += pos2;
        if score2 >= MAX_SCORE_1 {
            return score1 * die.count_rolled();
        }
    }
}

fn read_input(input: &mut dyn Read) -> (usize, usize) {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    let lines = buf.lines().collect::<Vec<_>>();
    let p1 = lines[0].split(": ").last().unwrap().parse::<usize>().unwrap();
    let p2 = lines[1].split(": ").last().unwrap().parse::<usize>().unwrap();

    (p1, p2)
}

pub fn part1(input: &mut dyn Read) -> String {
    let (p1, p2) = read_input(input);
    // println!("{}, {}", p1, p2);

    do_game::<DD100>(p1, p2).to_string()
}

// THREE_ROLLS[idx] is the number of possibilities that three rolls (each being one of 1,2,3) will sum up to idx
const THREE_ROLLS: [usize; 10] = [
    0, // 0
    0, // 1
    0, // 2
    1, // 3
    3, // 4
    3 + 3, // 5
    6 + 1, // 6
    3 + 3, // 7
    3, // 8
    1, // 9
];

// function (number_of_paths entering here) -> (number_of_paths leaving here)


// fn dp_solution(p1: usize, p2: usize) {
//     const MAX_SCORE_BOUND: usize = MAX_SCORE_2 + 10;
//
//     let mut DP = [[[[[(0, 0); MAX_POS]; MAX_POS]; MAX_SCORE_BOUND]; MAX_SCORE_BOUND]; 2];
//
//     for pos1 in 0..MAX_POS {
//         for pos2 in 0..MAX_POS {
//             for score2 in 0..MAX_SCORE_BOUND {
//                 // if it's p1's turn at those positions and p1 has >=21 score, then there's one possibility that he wins
//                 for score1 in 21..MAX_SCORE_BOUND {
//                     DP[0][pos1][pos2][score1][score2] = (1, 0);
//                 }
//
//             }
//         }
//     }
//
//     for pos1 in 0..MAX_POS {
//         for pos2 in 0..MAX_POS {
//             for score1 in 0..MAX_SCORE_BOUND {
//                 // if it's p2's turn at those positions and p1 has >=21 score, then there's one possibility that he wins
//                 for score2 in 21..MAX_SCORE_BOUND {
//                     DP[1][pos1][pos2][score1][score2] = (0, 1);
//                 }
//
//             }
//         }
//     }
//
//     for turn in 0..2 {
//         for pos1 in 0..MAX_POS {
//             for pos2 in 0..MAX_POS {
//                 for score1 in (0..21).rev() {
//                     for score2 in (0..21).rev() {
//
//                     }
//                 }
//             }
//         }
//     }
//
//
// }

const MAX_SCORE_2: usize = 1000;

fn number_of_possibilities_one_wins_starting_from(target: u8, cache: &mut HashMap<(u8, usize, usize, usize, usize), BigUint>, turn: u8, p1: usize, p2: usize, score1: usize, score2: usize) -> BigUint {
    // println!("Called with turn={}, p1={}, p2={}, score1={}, score2={}", turn, p1, p2, score1, score2);
    if let Some(score) = cache.get(&(turn, p1, p2, score1, score2)) {
        // println!("Got with params: target={}, p1={}, p2={}, score1={}, score2={}", target, p1, p2, score1, score2);
        return score.clone();
    }

    let possibilities = if turn == 1 {
        // if score1 >= 21 {
        //     return 1;
        // }
        if score2 >= MAX_SCORE_2 {
            BigUint::from((target == 2) as usize)
            // return 0;
        } else {
            let mut possibilities = BigUint::from(0u8);
            for num in 3..=9 {
                let new_pos = update_pos(p1, num);
                possibilities += number_of_possibilities_one_wins_starting_from(target, cache, 2, new_pos, p2, score1 + new_pos, score2) * BigUint::from(THREE_ROLLS[num]);
            }
            possibilities
        }


    } else {
        assert_eq!(turn, 2);

        if score1 >= MAX_SCORE_2 {
            BigUint::from((target == 1) as usize)
            // return 1;
        } else {
            let mut possibilities = BigUint::from(0u8);
            for num in 3..=9 {
                let new_pos = update_pos(p2, num);
                possibilities += BigUint::from(THREE_ROLLS[num]) * number_of_possibilities_one_wins_starting_from(target, cache, 1, p1, new_pos, score1, score2 + new_pos);
            }
            possibilities
        }
    };

    cache.insert((turn, p1, p2, score1, score2), possibilities.clone());

    possibilities
}

type HashMap<K, V> = FxHashMap<K, V>;

pub fn part2(input: &mut dyn Read) -> String {
    let (p1, p2) = read_input(input);
    // println!("{}, {}", p1, p2);

    let mut cache = HashMap::default();

    let option1 = number_of_possibilities_one_wins_starting_from(1, &mut cache,1, p1, p2, 0, 0);
    //  Clearing because target is different
    println!("option1 = {}", option1);

    cache.clear();
    let option2 = number_of_possibilities_one_wins_starting_from(2, &mut cache,1, p1, p2, 0, 0);

    // println!("option1 = {}", option1);
    println!("option2 = {}", option2);

    option1.max(option2).to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/21.txt");
    const REAL: &[u8] = include_bytes!("../inputs/21.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 739785);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 1067724);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 444356092776315usize);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 630947104784464usize);
    }
}