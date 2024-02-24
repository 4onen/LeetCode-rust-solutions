// https://leetcode.com/problems/valid-sudoku/

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        #[derive(Debug, Default, Clone, Copy)]
        struct Alphabet {
            bitset: u16,
        }
        impl Alphabet {
            fn add(&self, c: u8) -> Option<Self> {
                let bit = 1 << (c - b'0');
                if self.bitset & bit != 0 {
                    return None;
                } else {
                    Some(Alphabet {
                        bitset: self.bitset | bit,
                    })
                }
            }
        }
        // Rows
        for row in board.iter() {
            let mut alphabet = Alphabet::default();
            for &c in row.iter() {
                if let '.' = c {
                    continue;
                } else {
                    match alphabet.add(c as u8) {
                        Some(a) => alphabet = a,
                        None => return false,
                    }
                }
            }
        }
        // Cols
        for col in 0..9 {
            let mut alphabet = Alphabet::default();
            for row in 0..9 {
                let c = board[row][col];
                if let '.' = c {
                    continue;
                } else {
                    match alphabet.add(c as u8) {
                        Some(a) => alphabet = a,
                        None => return false,
                    }
                }
            }
        }
        // Squares
        for row in (0..9).step_by(3) {
            for col in (0..9).step_by(3) {
                let mut alphabet = Alphabet::default();
                for i in 0..3 {
                    for j in 0..3 {
                        let c = board[row + i][col + j];
                        if let '.' = c {
                            continue;
                        } else {
                            match alphabet.add(c as u8) {
                                Some(a) => alphabet = a,
                                None => return false,
                            }
                        }
                    }
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
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

    #[test]
    fn ex2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn my_invalid_row_ex() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '8'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn my_invalid_col_ex() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '1'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn discussion_case1() {
        let board = vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn discussion_case2() {
        let board = vec![
            vec!['.', '8', '7', '6', '5', '4', '3', '2', '1'],
            vec!['2', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['9', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }
}
