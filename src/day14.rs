use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;


pub fn part1(input: &mut dyn Read) -> String {
    let mut res = String::new();
    input.read_to_string(&mut res).unwrap();

    let mut lines = res.lines();
    let mut polymer = lines.next().unwrap().to_string();
    lines.next().unwrap();
    let mut rewrite_rules = lines.map(|l| {
        let v = l.split(" -> ").collect::<Vec<_>>();
        (v[0], v[1])
    }).collect::<Vec<_>>();

    rewrite_rules.sort();

    // let mut did_change = true;
    for i in 0..10 {
        // did_change = false;
        // Single step
        println!("Processing step {}", i);

        let mut new_polymer = String::new();
        let mut polymer_vec = polymer.chars().collect::<Vec<_>>();
        for wind in polymer_vec.windows(2) {
            if let [a, b] = wind {
                let str = format!("{}{}", a, b);
                new_polymer.push(*a);
                if let Ok(idx) = rewrite_rules.binary_search_by_key(&str.as_str(), |&(s, _)| s) {
                    // did_change = true;
                    let replacement = rewrite_rules[idx].1;
                    new_polymer.push_str(replacement);
                }
                // No, this will be pushed by the next iteration
                // new_polymer.push(*b);
            }
        }
        new_polymer.push(*polymer_vec.last().unwrap());

        println!("Replacing\n{}\nwith\n{}", polymer, new_polymer);

        polymer = new_polymer;

        // for &(pattern, insertion) in rewrite_rules.iter() {
        //     let pos = rewrite_rules.binary_search_by_key(pattern, )
        // }
    }

    let char_arr = "abcdefghijklmnopqrstuvwxyz".to_uppercase().chars().collect::<Vec<_>>();
    let occurences = char_arr.iter()
        .map(|&c| {
            polymer.chars().filter(|&d| d == c).count()
        })
        .filter(|&usize| usize > 0)
        .collect::<Vec<_>>();

    let min = occurences.iter().min().unwrap();
    let max = occurences.iter().max().unwrap();



    (max - min).to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut res = String::new();
    input.read_to_string(&mut res).unwrap();

    let mut lines = res.lines();

    let mut char_occs = [0usize; 26];
    let mut pattern_occs = [[0usize; 26]; 26];

    let mut prev_char = None;

    let mut polymer_chars = lines.next().unwrap().chars().map(|c| {
        let c = c as u8 - b'A';
        char_occs[c as usize] += 1;
        if let Some(prev_char) = prev_char {
            pattern_occs[prev_char as usize][c as usize] += 1;
        }
        prev_char = Some(c);
        c
    }).collect::<Vec<u8>>();
    lines.next().unwrap();

    let mut rewrite_rules_map = [[26; 26]; 26];
    for l in lines {
        // let mut parts = l.split(" -> ");
        let pat = &l[0..2]; //parts.next().unwrap();
        let mut pat_chars = pat.chars();
        let a = pat_chars.next().unwrap() as u8 - b'A';
        let b = pat_chars.next().unwrap() as u8 - b'A';

        let ins = l[6..7].chars().next().unwrap() as u8 - b'A';

        rewrite_rules_map[a as usize][b as usize] = ins;
    }

    // let mut rewrite_rules: Vec<([u8; 2], u8)> = lines.map(|l| {
    //     let v = l.split(" -> ").collect::<Vec<_>>();
    //     let pat = v[0].chars().collect::<Vec<_>>();
    //     ([pat[0] as u8 - b'A', pat[1] as u8 - b'A'], v[1].chars().next().unwrap() as u8 - b'A')
    // }).collect::<Vec<_>>();





    // for &(pat, ins) in &rewrite_rules {
    //     rewrite_rules_map[pat[0] as usize][pat[1] as usize] = ins;
    // }

    // let rewrite_rules = rewrite_rules.into_iter().collect::<HashMap<_,_>>();

    // let mut pattern_occs = rewrite_rules.iter()
    //     .map(|(&pat, _)| (pat, 0usize)).collect::<HashMap<_,_>>();


    // for wind in polymer_chars.windows(2) {
    //     if let &[a, b] = wind {
    //        pattern_occs[a as usize][b as usize] += 1;
    //     } else {
    //         unreachable!();
    //     }
    // }

    // println!("{:?}", pattern_occs);


    for step in 1..=40 {
        // println!("Processing step {}", step);
        let mut new_occs = [[0; 26]; 26];

        for a in 0..26 {
            for b in 0..26 {

                let ins = rewrite_rules_map[a as usize][b as usize];
                if ins < 26 {
                    let occ = pattern_occs[a as usize][b as usize];
                    if occ > 0 {
                        char_occs[ins as usize] += occ;
                        new_occs[a as usize][ins as usize] += occ;
                        new_occs[ins as usize][b as usize] += occ;
                    }
                }
                // else {
                //     // let occ = pattern_occs[a as usize][b as usize];
                //     // never entered
                //     // if occ > 0 {
                //     //     new_occs[a as usize][b as usize] = occ;
                //     // }
                // }
            }
        }

        pattern_occs = new_occs;


        // let mut new_occs = HashMap::new(); //pattern_occs.clone();
        // for (&[a,b], &occ) in pattern_occs.iter() {
        //     // println!("Processing pattern {:?}", [a,b]);
        //     if let Some(&ins) = rewrite_rules.get(&[a,b]) {
        //         // println!("Inserting {} inbetween {:?} which occured {} times", ins, [a,b], occ);
        //         *char_occs.get_mut(&ins).unwrap() += occ;
        //         // println!("char {} now has occs: {}", ins, char_occs[&ins]);
        //
        //         *new_occs.entry([a, ins]).or_insert(0) += occ;
        //         *new_occs.entry([ins, b]).or_insert(0) += occ;
        //
        //         // new_occs.insert([a, ins], occ);
        //         // new_occs.insert([ins, b], occ);
        //     } else {
        //         *new_occs.entry([a, b]).or_insert(0) += occ;
        //     }
        // }
        //
        // // println!("char occs after all patterns: {:?}", char_occs);
        //
        // pattern_occs = new_occs;

    }



    let mut min = usize::MAX;
    let mut max = usize::MIN;

    for occ in char_occs {
        if occ > 0 {
            min = min.min(occ);
        }
        max = max.max(occ);
    }

    // let min = char_occs.iter().filter(|&&occ| occ > 0).min().unwrap();
    // let max = char_occs.iter().max().unwrap();

    (max - min).to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/14.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 1588);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 2188189693529usize);
    }
}