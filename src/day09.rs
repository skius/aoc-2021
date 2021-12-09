use std::io::{BufRead, BufReader, Read};

fn adjacents(area: &Vec<Vec<u8>>, y: usize, x: usize) -> impl Iterator<Item = (usize, usize, u8)> {
    let mut adj = Vec::new();

    let n = area.len();
    let m = area[0].len();

    if y > 0 {
        adj.push((y - 1, x, area[y - 1][x]));
    }
    if y < n - 1 {
        adj.push((y + 1, x, area[y + 1][x]));
    }
    if x > 0 {
        adj.push((y, x - 1, area[y][x - 1]));
    }
    if x < m - 1 {
        adj.push((y, x + 1, area[y][x + 1]));
    }

    adj.into_iter()
}

fn get_area(input: &mut dyn Read) -> Vec<Vec<u8>> {
    BufReader::new(input).lines().map(|lines| {
        lines.unwrap().chars().map(|c| (c as u8) - ('0' as u8)).collect()
    }).collect::<Vec<Vec<u8>>>()
}

pub fn part1(input: &mut dyn Read) -> String {
    let area = get_area(input);
    let n = area.len();
    let m = area[0].len();

    let mut total = 0;
    for y in 0..n {
        for x in 0..m {
            let curr = area[y][x];
            if adjacents(&area, y, x).all(|(_, _, adj)| adj > curr) {
                total += curr as u32 + 1;
            }
        }
    }

    total.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let area = get_area(input);
    let n = area.len();
    let m = area[0].len();

    let mut visited = vec![vec![false; m as usize]; n as usize];
    let mut totals = vec![];

    for y in 0..n {
        for x in 0..m {
            if visited[y][x] {
                // Already handled this - don't need to push a 0
                continue;
            }
            let basin_size = dfs(&area, &mut visited, y, x);
            totals.push(basin_size);
        }
    }

    // Find top three in O(n)
    let (largest_idx, &largest_size) = totals.iter().enumerate().max_by_key(|(_, &size)| size).unwrap();
    totals.remove(largest_idx);
    let (middle_idx, &middle_size) = totals.iter().enumerate().max_by_key(|(_, &size)| size).unwrap();
    totals.remove(middle_idx);
    let (smallest_idx, &smallest_size) = totals.iter().enumerate().max_by_key(|(_, &size)| size).unwrap();
    totals.remove(smallest_idx);

    (largest_size * middle_size * smallest_size).to_string()
}

fn dfs(area: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, y: usize, x: usize) -> usize {
    if visited[y][x] || area[y][x] == 9 {
        return 0;
    }
    visited[y][x] = true;

    // 1 for the current cell + sum of all adjacent cells' sums
    1 + adjacents(area, y, x)
        .map(|(adj_y, adj_x, _)| {
            dfs(area, visited, adj_y as usize, adj_x as usize)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/09.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 15);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 1134);
    }
}