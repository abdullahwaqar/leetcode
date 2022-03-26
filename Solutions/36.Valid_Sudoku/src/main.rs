use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    // Each row must contain the digits 1-9 without repetition.
    // Each column must contain the digits 1-9 without repetition.
    // Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set_buffer = HashSet::with_capacity(9);
        // Rule 1
        for row in board.iter() {
            set_buffer.clear();
            for &element in row {
                if element != '.' {
                    if set_buffer.contains(&element) {
                        return false;
                    }
                    set_buffer.insert(element);
                }
            }
        }

        // Rule 2
        let mut grid = HashMap::with_capacity(9);
        for row_idx in 0..9 {
            for column_idx in 0..9 {
                let element = board[row_idx][column_idx];
                if element != '.' {
                    let set = grid.entry(column_idx).or_insert(HashSet::with_capacity(9));
                    if set.contains(&element) {
                        return false;
                    }
                    set.insert(element);
                }
            }
        }

        // Rule 3
        // 3 x 3
        grid.clear();
        for (row_idx, row) in board.iter().enumerate() {
            for (i, &element) in row.iter().enumerate() {
                if element != '.' {
                    let box_index = (row_idx / 3) * 3 + i / 3;
                    let set = grid.entry(box_index).or_insert(HashSet::with_capacity(9));
                    if set.contains(&element) {
                        return false;
                    }
                    set.insert(element);
                }
            }
        }

        return true;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ])
    );
    println!(
        "{:?}",
        Solution::is_valid_sudoku(vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']
        ])
    );
}
