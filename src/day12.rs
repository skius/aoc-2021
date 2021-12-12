use std::io::{BufRead, BufReader, Read};

fn parse_input(input: &mut dyn Read) -> Vec<Vec<String>> {
    BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        line.split("-").map(ToString::to_string).collect()
    }).collect()
}

// fn adj_matrix_from_edges(edges: &Vec<Vec<String>>) -> AdjMatrix {
//
// }

const START: Node = 0;
const END: Node = 1;

type AdjMatrix = Vec<Vec<bool>>;
type Node = usize;

pub fn part1(input: &mut dyn Read) -> String {
    let edges = parse_input(input);
    let mut small = edges.iter().flatten().filter(|s| *s == &s.to_ascii_lowercase()).collect::<Vec<_>>();
    small.dedup();
    let mut large = edges.iter().flatten().filter(|s| *s == &s.to_ascii_uppercase()).collect::<Vec<_>>();
    large.dedup();
    let num_small = small.len();
    let num_large = large.len();
    let n = num_small + num_large + 2;
    let mut adj_matrix = vec![vec![false; n]; n];

    // let start = "start".to_string();
    // let end = "end".to_string();
    // let idx_to_name = vec![vec![&start, &end], small.clone(), large.clone()].into_iter().flatten().collect::<Vec<_>>();

    for edge in &edges {
        let idx_src = match edge[0].as_ref() {
            "start" => START,
            "end" => END,
            s if s == s.to_lowercase() => small.iter().position(|name| *name == s).unwrap() + 2,
            s => large.iter().position(|name| *name == s).unwrap() + 2 + num_small,
        };

        let idx_target = match edge[1].as_ref() {
            "start" => START,
            "end" => END,
            s if s == s.to_lowercase() => small.iter().position(|name| *name == s).unwrap() + 2,
            s => large.iter().position(|name| *name == s).unwrap() + 2 + num_small,
        };

        adj_matrix[idx_src][idx_target] = true;
        adj_matrix[idx_target][idx_src] = true;
    }

    // print_adj(&adj_matrix, &idx_to_name);

    let mut visited = vec![false; n];
    dfs_1(num_small, &adj_matrix, &mut visited, START).to_string()
}

fn print_adj(adj: &AdjMatrix, idx_to_name: &Vec<&String>) {
    for (i, row) in adj.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col && i <= j {
                println!("{} -> {}", idx_to_name[i], idx_to_name[j]);
            }
        }
    }
}

// returns the number of paths to end
fn dfs_1(n_small: usize, adj: &AdjMatrix, visited: &mut Vec<bool>, node: Node) -> usize {
    if node < n_small + 2 && visited[node] {
        return 0;
    }
    if node == END {
        return 1;
    }
    visited[node] = true;

    let mut paths = 0;


    for (neighbor, _) in adj[node].iter().enumerate().filter(|(_, &b)| b) {
        let mut visited_clone = visited.clone();

        let neighbor_num = dfs_1(n_small, adj, &mut visited_clone, neighbor);
        if neighbor_num == 0 {
            continue;
        }

        paths += neighbor_num;
    }

    paths
}

pub fn part2(input: &mut dyn Read) -> String {
    let edges = parse_input(input);
    let mut small = edges.iter().flatten().filter(|s| *s == &s.to_ascii_lowercase()).filter(|s| *s != "start" && *s != "end").collect::<Vec<_>>();
    small.sort();
    small.dedup();
    let mut large = edges.iter().flatten().filter(|s| *s == &s.to_ascii_uppercase()).collect::<Vec<_>>();
    large.sort();
    large.dedup();
    let num_small = small.len();
    let num_large = large.len();
    let n = num_small + num_large + 2;
    let mut adj_matrix = vec![vec![false; n]; n];


    for edge in &edges {
        let idx_src = match edge[0].as_ref() {
            "start" => START,
            "end" => END,
            s if s == s.to_lowercase() => small.iter().position(|name| *name == s).unwrap() + 2,
            s => large.iter().position(|name| *name == s).unwrap() + 2 + num_small,
        };

        let idx_target = match edge[1].as_ref() {
            "start" => START,
            "end" => END,
            s if s == s.to_lowercase() => small.iter().position(|name| *name == s).unwrap() + 2,
            s => large.iter().position(|name| *name == s).unwrap() + 2 + num_small,
        };

        adj_matrix[idx_src][idx_target] = true;
        adj_matrix[idx_target][idx_src] = true;
    }

    let start = "start".to_string();
    let end = "end".to_string();
    let idx_to_name = vec![vec![&start, &end], small.clone(), large.clone()].into_iter().flatten().collect::<Vec<_>>();
    print_adj(&adj_matrix, &idx_to_name);

    let mut visited = vec![0; n];
    // visited[START] = 1;
    dfs_2(&idx_to_name, num_small, &adj_matrix, &mut visited, START).to_string()
}

fn dfs_2(idx_to_name: &Vec<&String>, n_small: usize, adj: &AdjMatrix, visited: &mut Vec<usize>, node: Node) -> usize {
    // println!("Visiting {}", idx_to_name[node]);
    // println!("[");
    // for (i, vis) in visited.iter().enumerate() {
    //     println!("{}: {}", idx_to_name[i], vis);
    // }
    // println!("]");

    // small caves
    if 2 <= node && node < n_small + 2 {
        if visited[node] == 2 {
            // no small cave may be visited more than twice
            return 0;
        }
        // but also if there is already a small cave that's been visited twice, then this one can't be visited more than once
        if visited.iter().take(n_small + 2).any(|&v| v == 2) && visited[node] == 1 {
            return 0;
        }
    }
    // start and end can be visited no more than once
    if node < 2 && visited[node] == 1 {
        // can't visit start,end twice
        return 0;
    }
    // println!("didn't abort");
    if node == END {
        // println!("found end, returning 1");
        return 1;
    }

    visited[node] += 1;


    let mut paths = 0;

    for (neighbor, _) in adj[node].iter().enumerate().filter(|(_, &b)| b) {
        let mut visited_clone = visited.clone();

        let neighbor_num = dfs_2(idx_to_name, n_small, adj, &mut visited_clone, neighbor);
        // if neighbor_num == 0 {
        //     continue;
        // }

        paths += neighbor_num;
    }

    // println!("found paths {}\n", paths);

    paths
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/12.txt");
    const SAMPLE_SMALL: &[u8] = include_bytes!("samples/12-small.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 226);
    }

    #[test]
    fn sample_small_part1() {
        test_implementation(part1, SAMPLE_SMALL, 10);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 3509);
    }

    #[test]
    fn sample_small_part2() {
        test_implementation(part2, SAMPLE_SMALL, 36);
    }
}