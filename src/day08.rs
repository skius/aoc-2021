use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};
use std::ops::Not;

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let mut lines = buf.lines();
    let mut total = 0;

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split("|");
        let input_part = parts.next().unwrap();
        let output_part = parts.next().unwrap();

        let input_displays = input_part.trim().split(" ").collect::<Vec<_>>();
        let output_displays = output_part.trim().split(" ").collect::<Vec<_>>();

        for display in &output_displays {
            let n = display.len();
            // println!("display: {}, length: {}", display, n);

            if n == 2 || n == 3 || n == 4 || n == 7 {
                total += 1;
            }
        }
    }

    total.to_string()
}

fn char_to_onehot(c: char) -> u8 {
    1 << (c as u8 - 'a' as u8)
}

fn onehot_to_char(onehot: u8) -> char {
    ((onehot as f64).log2() as u8 + ('a' as u8)) as char
}

fn set_diff(a: u8, b: u8) -> u8 {
    a & !b
}

fn set_union(a: u8, b: u8) -> u8 {
    a | b
}



pub fn part2(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let mut lines = buf.lines();
    let mut total = 0;

    let mut numbers_to_original_wires = [0; 10];
    numbers_to_original_wires[0] = "abcefg".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[1] = "cf".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[2] = "acdeg".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[3] = "acdfg".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[4] = "bdcf".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[5] = "abdfg".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[6] = "abdefg".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[7] = "acf".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[8] = "abcdefg".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);
    numbers_to_original_wires[9] = "abcdfg".chars().map(char_to_onehot).fold(0, |acc, curr| acc | curr);

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split("|");
        let input_part = parts.next().unwrap();
        let output_part = parts.next().unwrap();

        let input_displays = input_part.trim().split(" ").collect::<Vec<_>>();
        let output_displays = output_part.trim().split(" ").collect::<Vec<_>>();

        // println!("\n______________\n{}", line);

        // for display in &output_displays {
        //     let n = display.len();
        //     println!("display: {}, length: {}", display, n);
        //
        //     if n == 2 || n == 3 || n == 4 || n == 7 {
        //         total += 1;
        //     }
        // }

        // which wires are in which number
        let mut wires_in_number = [0; 10];//.map(|_| HashSet::<char>::new());
        // for each number, which current wire maps to which original wire
        let mut curr_to_original = HashMap::<char, char>::new();

        for display in &input_displays {
            let n = display.len();
            // don't need to loop over all chars, can just set equal to .collect
            for c in display.chars() {
                let c = char_to_onehot(c);
                match n {
                    2 | 3 | 4 | 7 => {
                        let num = match n {
                            2 => 1,
                            3 => 7,
                            4 => 4,
                            7 => 8,
                            _ => unreachable!(),
                        };
                        *wires_in_number.get_mut(num).unwrap() |= c;
                        // orders[num].insert(c, c);
                    }
                    _ => {}
                }
            }
        }

        // the wire a must be 7's wires minus 1's wires
        let a = set_diff(wires_in_number[7], wires_in_number[1]);
        // let curr_a = *a.iter().next().unwrap();
        curr_to_original.insert(onehot_to_char(a), 'a');

        let c_and_f = set_diff(wires_in_number[7], a);
        let b_and_d = set_diff(wires_in_number[4], c_and_f);
        let e_and_g = set_diff(wires_in_number[8], set_union(b_and_d, set_union(a, c_and_f)));
        // let b_and_d = wires_in_number[4].difference(&c_and_f).copied().collect::<HashSet<_>>();
        // let e_and_g = wires_in_number[8].difference(&b_and_d.union(&a.union(&c_and_f).copied().collect()).copied().collect()).copied().collect::<HashSet<_>>();


        // now a run to treat 0,6,9:
        for display in &input_displays {
            let n = display.len();
            if n != 6 {
                continue;
            }

            let wires = display.chars().map(char_to_onehot).fold(0, |acc, onehot| acc | onehot);
            // if it's just missing e, then it's 9
            let diff_e_g = set_diff(wires,e_and_g);
            let diff_len = diff_e_g.count_ones();
            if diff_len == 5 {
                // only 9 will still have 5 left over
                wires_in_number[9] = wires;
                continue;
            }

            // if it's just missing c, then it's 9
            let diff_c_f = set_diff(wires, c_and_f);
            let diff_len = diff_c_f.count_ones();
            if diff_len == 5 {
                // only 6 will still have 5 left over
                wires_in_number[6] = wires;
                continue;
            }

            // if it's just missing d, then it's 9
            let diff_b_d = set_diff(wires, b_and_d);
            let diff_len = diff_b_d.count_ones();
            if diff_len == 5 {
                // only 0 will still have 5 left over
                wires_in_number[0] = wires;
                continue;
            }
        }

        // we have more mappings now
        let d = set_diff(b_and_d, wires_in_number[0]);
        curr_to_original.insert(onehot_to_char(d), 'd');
        let b = set_diff(b_and_d, d);
        curr_to_original.insert(onehot_to_char(b), 'b');

        let e = set_diff(e_and_g, wires_in_number[9]);
        curr_to_original.insert(onehot_to_char(e), 'e');
        let g = set_diff(e_and_g, e);
        curr_to_original.insert(onehot_to_char(g), 'g');

        let c = set_diff(c_and_f, wires_in_number[6]);
        curr_to_original.insert(onehot_to_char(c), 'c');
        let f = set_diff(c_and_f, c);
        curr_to_original.insert(onehot_to_char(f), 'f');

        // now we have all the mappings
        let mut curr_total = 0;
        for display in &output_displays {
            let wires = display.chars().map(|c| curr_to_original[&c]).map(char_to_onehot).fold(0, |acc, onehot| acc | onehot);
            // println!("output_display curr wires: {}, supposed original: {}", display, wires.iter().collect::<String>());

            let (idx, _) = numbers_to_original_wires.iter().enumerate().find(|(idx, original_wires)| &wires == *original_wires).unwrap();
            curr_total *= 10;
            curr_total += idx;
        }

        total += curr_total;
    }

    total.to_string()
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/08.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 26);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 61229);
    }
}