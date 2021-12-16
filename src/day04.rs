use std::io::{BufRead, BufReader, Read};

pub fn part1(input: &mut dyn Read) -> String {
    let buf = BufReader::new(input);
    let mut lines = buf.lines();
    let order = lines.next().unwrap().unwrap().split(",").map(|num| num.parse::<usize>().unwrap()).collect::<Vec<_>>();
    lines.next().unwrap();

    let mut lines = lines.peekable();

    let mut boards = Vec::<[[usize; 5]; 5]>::new();

    while let Some(line) = lines.peek() {
        // println!("");
        let mut board = [[300; 5]; 5];
        for row in 0..5 {
            let line = lines.next().unwrap().unwrap();
            // println!("line: {}", line);
            for (col, num) in line.split_whitespace().enumerate() {
                board[row][col] = num.parse::<usize>().unwrap();
            }
        }

        boards.push(board);

        lines.next().unwrap();
    }

    // println!("{:?}", boards);

    for pick in order {
        boards = boards.into_iter().map(|mut board| {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == pick {
                        // mark as picked
                        board[i][j] = 100 + pick;
                    }
                }
            }
            board
        }).collect();

        // println!("after pick {} boards look like this:\n{:?}\n", pick, boards);

        if let Some((unpicked_sum, _winner_board)) = find_winner(&boards) {
            return (unpicked_sum * pick).to_string();
        }
    }

    (-1).to_string()
}

fn find_winner(boards: &Vec<[[usize; 5]; 5]>) -> Option<(usize, &[[usize; 5]; 5])> {
    // let mut winner_idx = -1;

    let winner_board = boards.iter().find(|board| {
        // check if there's any row with only picked numbers
        if board.iter().any(|row| row.iter().all(|num| *num >= 100)) {
            return true;
        }

        // check if there's any column with only picked numbers
        if (0..5).into_iter().any(|col_idx| (0..5).into_iter().all(|row_idx| board[row_idx][col_idx] >= 100)) {
            return true;
        }

        false
    })?;

    // println!("winner board: {:?}", winner_board);

    let sum = winner_board.iter().flatten().filter(|num| **num < 100).sum::<usize>();

    Some((sum, winner_board))
}

fn remove_all_winners(boards: Vec<[[usize; 5]; 5]>) -> Vec<[[usize; 5]; 5]> {
    boards.into_iter().filter(|board| {
        // check if there's any row with only picked numbers
        if board.iter().any(|row| row.iter().all(|num| *num >= 100)) {
            return false;
        }

        // check if there's any column with only picked numbers
        if (0..5).into_iter().any(|col_idx| (0..5).into_iter().all(|row_idx| board[row_idx][col_idx] >= 100)) {
            return false;
        }

        true
    }).collect()

}

pub fn part2(input: &mut dyn Read) -> String {
    let buf = BufReader::new(input);
    let mut lines = buf.lines();
    let order = lines.next().unwrap().unwrap().split(",").map(|num| num.parse::<usize>().unwrap()).collect::<Vec<_>>();
    lines.next().unwrap();

    let mut lines = lines.peekable();

    let mut boards = Vec::<[[usize; 5]; 5]>::new();

    while let Some(line) = lines.peek() {
        // println!("");
        let mut board = [[300; 5]; 5];
        for row in 0..5 {
            let line = lines.next().unwrap().unwrap();
            // println!("line: {}", line);
            for (col, num) in line.split_whitespace().enumerate() {
                board[row][col] = num.parse::<usize>().unwrap();
            }
        }

        boards.push(board);

        lines.next().unwrap();
    }

    // println!("{:?}", boards);

    for pick in order {
        boards = boards.into_iter().map(|mut board| {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == pick {
                        // mark as picked
                        board[i][j] = 100 + pick;
                    }
                }
            }
            board
        }).collect();

        //println!("after pick {} boards look like this:\n{:?}\n", pick, boards);

        let sum = boards[0].iter().flatten().filter(|num| **num < 100).sum::<usize>();
        boards = remove_all_winners(boards);
        if boards.len() == 0 {
            // then the previous board (sum) must have been the last winner
            return (sum * pick).to_string();
        }

    }

    (-1).to_string()
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/04.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 4512);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 1924);
    }
}

