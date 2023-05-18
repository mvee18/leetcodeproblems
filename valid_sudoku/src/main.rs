use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = HashMap::<usize, Vec<char>>::with_capacity(9);
        let mut cols = HashMap::<usize, Vec<char>>::with_capacity(9);
        let mut boxes = HashMap::<(usize, usize), Vec<char>>::with_capacity(9);

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    continue;
                }

                let box_index = (r / 3, c / 3);

                // First, try to get the corresponding indices.
                // If they don't exist, create them.
                rows.entry(r).or_insert(Vec::new());
                cols.entry(c).or_insert(Vec::new());
                boxes.entry(box_index).or_insert(Vec::new());

                // See if any of the indices contain the value. If so, return false.
                if rows[&r].contains(&board[r][c])
                    || cols[&c].contains(&board[r][c])
                    || boxes[&box_index].contains(&board[r][c])
                {
                    return false;
                }

                // Add the value to the indices.
                rows.get_mut(&r).unwrap().push(board[r][c]);
                cols.get_mut(&c).unwrap().push(board[r][c]);
                boxes.get_mut(&box_index).unwrap().push(board[r][c]);
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_by_three() {
        let board = vec![
            vec!['5', '3', '.'],
            vec!['6', '.', '.'],
            vec!['.', '9', '8'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }
}
