use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

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
            println!("display: {}, length: {}", display, n);

            if n == 2 || n == 3 || n == 4 || n == 7 {
                total += 1;
            }
        }
    }

    total.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let mut lines = buf.lines();
    let mut total = 0;

    let mut numbers_to_original_wires = Vec::<HashSet<char>>::new();
    numbers_to_original_wires.insert(0, "abcefg".chars().collect());
    numbers_to_original_wires.insert(1, "cf".chars().collect());
    numbers_to_original_wires.insert(2, "acdeg".chars().collect());
    numbers_to_original_wires.insert(3, "acdfg".chars().collect());
    numbers_to_original_wires.insert(4, "bdcf".chars().collect());
    numbers_to_original_wires.insert(5, "abdfg".chars().collect());
    numbers_to_original_wires.insert(6, "abdefg".chars().collect());
    numbers_to_original_wires.insert(7, "acf".chars().collect());
    numbers_to_original_wires.insert(8, "abcdefg".chars().collect());
    numbers_to_original_wires.insert(9, "abcdfg".chars().collect());

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split("|");
        let input_part = parts.next().unwrap();
        let output_part = parts.next().unwrap();

        let input_displays = input_part.trim().split(" ").collect::<Vec<_>>();
        let output_displays = output_part.trim().split(" ").collect::<Vec<_>>();

        println!("\n______________\n{}", line);

        // for display in &output_displays {
        //     let n = display.len();
        //     println!("display: {}, length: {}", display, n);
        //
        //     if n == 2 || n == 3 || n == 4 || n == 7 {
        //         total += 1;
        //     }
        // }

        // which wires are in which number
        let mut wires_in_number = [0; 10].map(|_| HashSet::<char>::new());
        // for each number, which current wire maps to which original wire
        let mut curr_to_original = HashMap::<char, char>::new();

        for display in &input_displays {
            let n = display.len();
            // don't need to loop over all chars, can just set equal to .collect
            for c in display.chars() {
                match n {
                    2 | 3 | 4 | 7 => {
                        let num = HashMap::from([
                            (2, 1),
                            (3, 7),
                            (4, 4),
                            (7, 8),
                        ])[&n];
                        wires_in_number[num].insert(c);
                        // orders[num].insert(c, c);
                    }
                    _ => {}
                }
            }
        }
        println!("Hashsets: {:?}", wires_in_number);

        assert!(wires_in_number[1].len() != 0);
        assert!(wires_in_number[7].len() != 0);

        // the wire a must be 7's wires minus 1's wires
        let a = wires_in_number[7].difference(&wires_in_number[1]).copied().collect::<HashSet<_>>();
        let curr_a = *a.iter().next().unwrap();
        curr_to_original.insert(curr_a, 'a');

        let c_and_f = wires_in_number[7].difference(&a).copied().collect::<HashSet<_>>();
        let b_and_d = wires_in_number[4].difference(&c_and_f).copied().collect::<HashSet<_>>();
        let e_and_g = wires_in_number[8].difference(&b_and_d.union(&a.union(&c_and_f).copied().collect()).copied().collect()).copied().collect::<HashSet<_>>();


        // now a run to treat 0,6,9:
        for display in &input_displays {
            let n = display.len();
            if n != 6 {
                continue;
            }

            let wires = display.chars().collect::<HashSet<_>>();
            // if it's just missing e, then it's 9
            let diff_e_g = wires.difference(&e_and_g).collect::<HashSet<_>>();
            let diff_len = diff_e_g.len();
            if diff_len == 5 {
                // only 9 will still have 5 left over
                wires_in_number[9] = wires;
                continue;
            }

            // if it's just missing c, then it's 9
            let diff_c_f = wires.difference(&c_and_f).collect::<HashSet<_>>();
            let diff_len = diff_c_f.len();
            if diff_len == 5 {
                // only 6 will still have 5 left over
                wires_in_number[6] = wires;
                continue;
            }

            // if it's just missing d, then it's 9
            let diff_b_d = wires.difference(&b_and_d).collect::<HashSet<_>>();
            let diff_len = diff_b_d.len();
            if diff_len == 5 {
                // only 0 will still have 5 left over
                wires_in_number[0] = wires;
                continue;
            }
        }

        // we have more mappings now
        let d = b_and_d.difference(&wires_in_number[0]).copied().collect::<HashSet<_>>();
        curr_to_original.insert(*d.iter().next().unwrap(), 'd');
        let b = b_and_d.difference(&d).copied().collect::<HashSet<_>>();
        curr_to_original.insert(*b.iter().next().unwrap(), 'b');

        let e = e_and_g.difference(&wires_in_number[9]).copied().collect::<HashSet<_>>();
        curr_to_original.insert(*e.iter().next().unwrap(), 'e');
        let g = e_and_g.difference(&e).copied().collect::<HashSet<_>>();
        curr_to_original.insert(*g.iter().next().unwrap(), 'g');

        let c = c_and_f.difference(&wires_in_number[6]).copied().collect::<HashSet<_>>();
        curr_to_original.insert(*c.iter().next().unwrap(), 'c');
        let f = c_and_f.difference(&c).copied().collect::<HashSet<_>>();
        curr_to_original.insert(*f.iter().next().unwrap(), 'f');

        // now we have all the mappings
        let mut curr_total = 0;
        for display in &output_displays {
            let wires = display.chars().map(|c| curr_to_original[&c]).collect::<HashSet<_>>();
            println!("output_display curr wires: {}, supposed original: {}", display, wires.iter().collect::<String>());

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