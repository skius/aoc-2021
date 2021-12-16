use std::io::{BufRead, BufReader, Read};

pub fn part1(input: &mut dyn Read) -> String {
    let reader = BufReader::new(input);
    let stuff = reader
        .lines()
        .map(|line| {
            line.unwrap().chars().collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let total = stuff.len();

    let sums = stuff.iter()
        .fold(vec![0; stuff[0].len()], |mut acc, data| {
            for (i, &c) in data.iter().enumerate() {
                match c {
                    '1' => {
                        acc[i] += 1;
                    }
                    _ => {},
                }
            };
            acc
        });

    let gamma = sums.iter().map(|&sum| {
        if sum > total/2 {
            '1'
        } else {
            '0'
        }
    }).collect::<String>();

    let epsilon = sums.iter().map(|&sum| {
        if sum < total/2 {
            '1'
        } else {
            '0'
        }
    }).collect::<String>();



    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    (gamma*epsilon).to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let reader = BufReader::new(input);
    let bits_orig = reader
        .lines()
        .map(|line| {
            line.unwrap().chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let mut bits = bits_orig.clone();

    let mut bit_idx = 0;

    while bits.len() > 1 {
        // println!("oxygen stuff: {:?}", bits);

        let sum = bits.iter()
            .map(|v| v[bit_idx])
            .sum::<usize>();

        // println!("sum: {}", sum);

        let keep = if sum > (bits.len() - 1)/2 {
            // keep ones
            1
        } else {
            0
        };

        bits = bits.into_iter().filter(|v| v[bit_idx] == keep).collect::<Vec<_>>();

        bit_idx += 1;
    }

    let oxygen = u64::from_str_radix(&bits[0].iter().map(|&b| b.to_string()).collect::<String>(), 2).unwrap();

    // println!("oxygen: {}, binary: {}", oxygen, &bits[0].iter().map(|&b| b.to_string()).collect::<String>());


    let mut bits = bits_orig.clone();

    let mut bit_idx = 0;

    while bits.len() > 1 {
        // println!("co2 stuff: {:?}", bits);

        let sum = bits.iter()
            .map(|v| v[bit_idx])
            .sum::<usize>();

        let keep = if sum < (bits.len() + 1)/2 {
            // one is less common, keep ones
            1
        } else {
            0
        };

        bits = bits.into_iter().filter(|v| v[bit_idx] == keep).collect::<Vec<_>>();

        bit_idx += 1;
    }

    let co2 = u64::from_str_radix(&bits[0].iter().map(|&b| b.to_string()).collect::<String>(), 2).unwrap();
    // println!("co2: {}, binary: {}", co2, &bits[0].iter().map(|&b| b.to_string()).collect::<String>());

    (oxygen * co2).to_string()
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/03.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 198);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 230);
    }
}
