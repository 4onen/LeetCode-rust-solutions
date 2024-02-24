// https://leetcode.com/problems/sudoku-solver/

pub struct Solution;

// Incomplete Direct sol'n
// impl Solution {
//     pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
//         use std::ops::BitAnd;
//         // A bitset representing the digits 1-9. The bit for digit `i` is set if
//         // and only if the digit `i` is present in the set.
//         #[derive(Debug, Default, Clone, Copy)]
//         struct Alphabet {
//             bitset: u16,
//         }
//         impl Alphabet {
//             const COMPLETE: Alphabet = Alphabet {
//                 bitset: 0b111_111_111,
//             };
//             const fn add(&self, c: u8) -> Option<Self> {
//                 let bit = 1 << (c - b'0');
//                 if self.bitset & bit != 0 {
//                     return None;
//                 } else {
//                     Some(Alphabet {
//                         bitset: self.bitset | bit,
//                     })
//                 }
//             }
//             const fn remove(&self, c: u8) -> Self {
//                 let bit = 1 << (c - b'0');
//                 Alphabet {
//                     bitset: self.bitset & !bit,
//                 }
//             }
//         }
//         impl From<u16> for Alphabet {
//             fn from(bitset: u16) -> Self {
//                 Alphabet { bitset }
//             }
//         }
//         // Step 1: Compute the initial alphabet for each row, column, and box.
//         // We'll merge these alphabets bitwise rather than computing them from
//         // scratch over and over for each cell.
//         let mut row_alphabets = [Alphabet::default(); 9];
//         for (i, row) in board.iter().enumerate() {
//             for &c in row {
//                 if c != '.' {
//                     row_alphabets[i] = row_alphabets[i].add(c as u8).unwrap();
//                 }
//             }
//         }
//         let mut col_alphabets = [Alphabet::default(); 9];
//         for row in board.iter() {
//             for col_idx in 0..9 {
//                 let c = row[col_idx];
//                 if c != '.' {
//                     col_alphabets[col_idx] = col_alphabets[col_idx].add(c as u8).unwrap();
//                 }
//             }
//         }
//         let mut box_alphabets = [[Alphabet::default(); 3]; 3];
//         for (i, box_row) in (0..9).step_by(3).enumerate() {
//             for (j, box_col) in (0..9).step_by(3).enumerate() {
//                 for row_idx in 0..3 {
//                     for col_idx in 0..3 {
//                         let c = board[box_row + row_idx][box_col + col_idx];
//                         if c != '.' {
//                             box_alphabets[i][j] = box_alphabets[i][j].add(c as u8).unwrap();
//                         }
//                     }
//                 }
//             }
//         }
//         // Step 2: Merge the alphabets together to get the initial alphabet for
//         // each cell.
//         let mut cell_alphabets = [[Alphabet::default(); 9]; 9];
//         for row_idx in 0..9 {
//             for col_idx in 0..9 {
//                 cell_alphabets[row_idx][col_idx] = row_alphabets[row_idx]
//                     .bitset
//                     .bitand(col_alphabets[col_idx].bitset)
//                     .bitand(box_alphabets[row_idx / 3][col_idx / 3].bitset)
//                     .into();
//             }
//         }
//         // Step 3: Find any empty cell with a size-1 alphabet and fill it in.
//         fn fill_size1_groups(
//             board: &mut Vec<Vec<char>>,
//             cell_alphabets: &mut [[Alphabet; 9]; 9],
//         ) -> bool {
//             let mut filled = false;
//             for row_idx in 0..9 {
//                 for col_idx in 0..9 {
//                     if board[row_idx][col_idx] != '.' {
//                         continue;
//                     }
//                     let alphabet = cell_alphabets[row_idx][col_idx];
//                     if alphabet.bitset.count_ones() == 1 {
//                         let c = alphabet.bitset.trailing_zeros() as u8 + b'1';
//                         board[row_idx][col_idx] = c as char;
//                         cell_alphabets[row_idx]
//                             .iter_mut()
//                             .for_each(|a| *a = a.remove(c));
//                         cell_alphabets
//                             .iter_mut()
//                             .for_each(|row| row[col_idx] = row[col_idx].remove(c));
//                         let box_row = row_idx / 3;
//                         let box_col = col_idx / 3;
//                         for row_idx in 0..3 {
//                             for col_idx in 0..3 {
//                                 cell_alphabets[box_row * 3 + row_idx][box_col * 3 + col_idx] =
//                                     cell_alphabets[box_row * 3 + row_idx][box_col * 3 + col_idx]
//                                         .remove(c);
//                             }
//                         }
//                         filled = true;
//                     }
//                 }
//             }
//             filled
//         }
//         while fill_size1_groups(board, &mut cell_alphabets) {}
//         // Step 4: Find groups of cells that share a row, column, or box and
//         // have the same alphabet. Remove that alphabet from the alphabet of
//         // all other cells in the group.
//     }
// }

// Direct Backtracking sol'n
// impl Solution {
//     pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
//         fn valid_for_space(board: &[Vec<char>], c: u8, row_idx: u8, col_idx: u8) -> bool {
//             let row = &board[row_idx as usize];
//             let box_row = row_idx / 3 * 3;
//             let box_col = col_idx / 3 * 3;
//             for i in 0..9 {
//                 if row[i as usize] == c as char {
//                     return false;
//                 }
//                 if board[i as usize][col_idx as usize] == c as char {
//                     return false;
//                 }
//                 if board[(box_row + i / 3) as usize][(box_col + i % 3) as usize] == c as char {
//                     return false;
//                 }
//             }
//             true
//         }
//         fn backtrack(board: &mut [Vec<char>], mut idx: u8) -> bool {
//             let mut row_idx = idx / 9;
//             let mut col_idx = idx % 9;
//             loop {
//                 if idx >= 81 {
//                     return true;
//                 }
//                 if board[row_idx as usize][col_idx as usize] == '.' {
//                     break;
//                 }
//                 idx += 1;
//                 row_idx = idx / 9;
//                 col_idx = idx % 9;
//             }
//             for c in '1'..='9' {
//                 if valid_for_space(&board, c as u8, row_idx, col_idx) {
//                     board[row_idx as usize][col_idx as usize] = c;
//                     if backtrack(board, idx + 1) {
//                         return true;
//                     }
//                     board[row_idx as usize][col_idx as usize] = '.';
//                 }
//             }
//             false
//         }
//         backtrack(board, 0);
//     }
// }

// Backtracking on copied board
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        // First, copy the board into 81 bytes of stack memory
        // One cache line, should be very, very fast.
        let mut hot_board = [[0u8; 9]; 9];
        for (i, row) in board.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c != '.' {
                    hot_board[i][j] = c as u8 - b'0';
                }
            }
        }
        // Second, define a function to check if a digit is valid for a given
        // row, column, or box in the hot board.
        fn valid_for_space(hot_board: &[[u8; 9]; 9], c: u8, row_idx: u8, col_idx: u8) -> bool {
            let row = &hot_board[row_idx as usize];
            let box_row = row_idx / 3 * 3;
            let box_col = col_idx / 3 * 3;
            for i in 0..9 {
                if row[i as usize] == c {
                    return false;
                }
                if hot_board[i as usize][col_idx as usize] == c {
                    return false;
                }
                if hot_board[(box_row + i / 3) as usize][(box_col + i % 3) as usize] == c {
                    return false;
                }
            }
            true
        }
        // Third, define a function to backtrack on the hot board.
        fn backtrack(hot_board: &mut [[u8; 9]; 9], mut idx: u8) -> bool {
            let mut row_idx = idx / 9;
            let mut col_idx = idx % 9;
            loop {
                if idx >= 81 {
                    return true;
                }
                if hot_board[row_idx as usize][col_idx as usize] == 0 {
                    break;
                }
                idx += 1;
                row_idx = idx / 9;
                col_idx = idx % 9;
            }
            for c in 1..=9 {
                if valid_for_space(&hot_board, c, row_idx, col_idx) {
                    hot_board[row_idx as usize][col_idx as usize] = c;
                    if backtrack(hot_board, idx + 1) {
                        return true;
                    }
                    hot_board[row_idx as usize][col_idx as usize] = 0;
                }
            }
            false
        }
        // Fourth, solve.
        backtrack(&mut hot_board, 0);
        // Fifth, copy the hot board back into the original board.
        for (i, row) in board.iter_mut().enumerate() {
            for (j, c) in row.iter_mut().enumerate() {
                *c = (hot_board[i][j] + b'0') as char;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut input = vec![
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
        let expected = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_example1() {
        let mut input = vec![
            vec!['2', '6', '.', '.', '7', '.', '.', '.', '.'],
            vec!['.', '.', '9', '6', '.', '2', '.', '1', '.'],
            vec!['4', '.', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '.', '.', '.', '.', '.', '8'],
            vec!['8', '.', '7', '9', '.', '4', '5', '.', '2'],
            vec!['9', '.', '.', '.', '.', '.', '7', '.', '.'],
            vec!['.', '.', '.', '.', '.', '7', '.', '.', '5'],
            vec!['.', '4', '.', '2', '.', '6', '1', '.', '.'],
            vec!['.', '.', '.', '.', '3', '.', '.', '8', '6'],
        ];
        let expected = vec![
            vec!['2', '6', '1', '4', '7', '8', '3', '5', '9'],
            vec!['3', '7', '9', '6', '5', '2', '8', '1', '4'],
            vec!['4', '8', '5', '3', '1', '9', '6', '2', '7'],
            vec!['6', '5', '3', '7', '2', '1', '9', '4', '8'],
            vec!['8', '1', '7', '9', '6', '4', '5', '3', '2'],
            vec!['9', '2', '4', '5', '8', '3', '7', '6', '1'],
            vec!['1', '3', '6', '8', '4', '7', '2', '9', '5'],
            vec!['5', '4', '8', '2', '9', '6', '1', '7', '3'],
            vec!['7', '9', '2', '1', '3', '5', '4', '8', '6'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_example2() {
        let mut input = vec![
            vec!['6', '.', '.', '.', '.', '.', '8', '.', '3'],
            vec!['.', '4', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '4', '.', '7', '.'],
            vec!['3', '.', '.', '2', '.', '.', '.', '.', '.'],
            vec!['1', '.', '6', '.', '.', '.', '.', '.', '.'],
            vec!['.', '2', '.', '.', '.', '.', '.', '5', '.'],
            vec!['.', '.', '.', '.', '8', '.', '6', '.', '.'],
            vec!['.', '.', '.', '.', '1', '.', '.', '.', '.'],
        ];
        let expected = vec![
            vec!['6', '1', '7', '4', '5', '9', '8', '2', '3'],
            vec!['2', '4', '8', '7', '3', '6', '9', '1', '5'],
            vec!['5', '3', '9', '1', '2', '8', '4', '6', '7'],
            vec!['9', '8', '2', '5', '6', '4', '3', '7', '1'],
            vec!['3', '7', '4', '2', '9', '1', '5', '8', '6'],
            vec!['1', '5', '6', '8', '7', '3', '2', '9', '4'],
            vec!['8', '2', '3', '6', '4', '7', '1', '5', '9'],
            vec!['7', '9', '1', '3', '8', '5', '6', '4', '2'],
            vec!['4', '6', '5', '9', '1', '2', '7', '3', '8'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_example3() {
        let mut input = vec![
            vec!['.', '.', '5', '.', '.', '.', '9', '8', '7'],
            vec!['.', '4', '.', '.', '5', '.', '.', '.', '1'],
            vec!['.', '.', '7', '.', '.', '.', '.', '.', '.'],
            vec!['2', '.', '.', '.', '4', '8', '.', '.', '.'],
            vec!['.', '9', '.', '1', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '2', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '6', '.', '.', '2', '.', '.'],
            vec!['.', '.', '.', '.', '.', '9', '.', '7', '.'],
            vec!['.', '.', '.', '.', '.', '.', '5', '.', '.'],
        ];
        let expected = vec![
            vec!['1', '3', '5', '4', '2', '6', '9', '8', '7'],
            vec!['8', '4', '6', '9', '5', '7', '3', '2', '1'],
            vec!['9', '2', '7', '3', '8', '1', '4', '6', '5'],
            vec!['2', '1', '3', '7', '4', '8', '6', '5', '9'],
            vec!['5', '9', '8', '1', '6', '3', '7', '4', '2'],
            vec!['6', '7', '4', '2', '9', '5', '8', '1', '3'],
            vec!['3', '5', '1', '6', '7', '4', '2', '9', '8'],
            vec!['4', '8', '2', '5', '3', '9', '1', '7', '6'],
            vec!['7', '6', '9', '8', '1', '2', '5', '3', '4'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_example4() {
        let mut input = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '7', '1'],
            vec!['.', '2', '.', '8', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '4', '.', '3', '.', '.', '.'],
            vec!['7', '.', '.', '.', '6', '.', '.', '5', '.'],
            vec!['.', '.', '.', '2', '.', '.', '3', '.', '.'],
            vec!['9', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '.', '7', '.', '.', '.', '.'],
            vec!['.', '8', '.', '.', '.', '.', '4', '.', '.'],
            vec!['.', '.', '.', '.', '5', '.', '.', '.', '.'],
        ];
        let expected = vec![
            vec!['3', '4', '9', '5', '2', '6', '8', '7', '1'],
            vec!['5', '2', '1', '8', '9', '7', '6', '4', '3'],
            vec!['8', '7', '6', '4', '1', '3', '5', '2', '9'],
            vec!['7', '1', '8', '3', '6', '9', '2', '5', '4'],
            vec!['4', '6', '5', '2', '8', '1', '3', '9', '7'],
            vec!['9', '3', '2', '7', '4', '5', '1', '8', '6'],
            vec!['6', '5', '4', '1', '7', '8', '9', '3', '2'],
            vec!['1', '8', '7', '9', '3', '2', '4', '6', '5'],
            vec!['2', '9', '3', '6', '5', '4', '7', '1', '8'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_example5() {
        let mut input = vec![
            vec!['.', '4', '7', '.', '8', '.', '.', '.', '1'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '6', '.', '.', '7', '.', '.'],
            vec!['6', '.', '.', '.', '.', '3', '5', '7', '.'],
            vec!['.', '.', '.', '.', '.', '5', '.', '.', '.'],
            vec!['.', '1', '.', '.', '6', '.', '.', '.', '.'],
            vec!['2', '8', '.', '.', '4', '.', '.', '.', '.'],
            vec!['.', '9', '.', '1', '.', '.', '.', '4', '.'],
            vec!['.', '.', '.', '.', '2', '.', '6', '9', '.'],
        ];
        let expected = vec![
            vec!['9', '4', '7', '5', '8', '2', '3', '6', '1'],
            vec!['8', '6', '3', '4', '7', '1', '9', '5', '2'],
            vec!['1', '5', '2', '6', '3', '9', '7', '8', '4'],
            vec!['6', '2', '4', '8', '1', '3', '5', '7', '9'],
            vec!['7', '3', '8', '2', '9', '5', '4', '1', '6'],
            vec!['5', '1', '9', '7', '6', '4', '8', '2', '3'],
            vec!['2', '8', '5', '9', '4', '6', '1', '3', '7'],
            vec!['3', '9', '6', '1', '5', '7', '2', '4', '8'],
            vec!['4', '7', '1', '3', '2', '8', '6', '9', '5'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_example6() {
        let mut input = vec![
            vec!['.', '2', '.', '.', '.', '.', '.', '.', '.'],
            vec!['3', '.', '5', '.', '6', '2', '.', '.', '9'],
            vec!['.', '6', '8', '.', '.', '.', '3', '.', '.'],
            vec!['.', '5', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '6', '4', '.', '8', '.', '2'],
            vec!['.', '.', '4', '7', '.', '.', '9', '.', '.'],
            vec!['.', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '.', '.', '6', '.', '.', '.'],
            vec!['1', '7', '.', '4', '3', '.', '.', '.', '.'],
        ];
        let expected = vec![
            vec!['4', '2', '7', '5', '9', '3', '1', '8', '6'],
            vec!['3', '1', '5', '8', '6', '2', '4', '7', '9'],
            vec!['9', '6', '8', '1', '7', '4', '3', '2', '5'],
            vec!['6', '5', '9', '3', '2', '8', '7', '1', '4'],
            vec!['7', '3', '1', '6', '4', '9', '8', '5', '2'],
            vec!['2', '8', '4', '7', '5', '1', '9', '6', '3'],
            vec!['5', '9', '3', '2', '8', '7', '6', '4', '1'],
            vec!['8', '4', '2', '9', '1', '6', '5', '3', '7'],
            vec!['1', '7', '6', '4', '3', '5', '2', '9', '8'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_example7() {
        let mut input = vec![
            vec!['1', '.', '.', '.', '.', '6', '.', '8', '.'],
            vec!['.', '6', '4', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '4', '.', '.', '.', '7'],
            vec!['.', '.', '.', '.', '9', '.', '6', '.', '.'],
            vec!['.', '7', '.', '4', '.', '.', '5', '.', '.'],
            vec!['5', '.', '.', '.', '7', '.', '1', '.', '.'],
            vec!['.', '5', '.', '.', '.', '.', '3', '2', '.'],
            vec!['3', '.', '.', '.', '.', '8', '.', '.', '.'],
            vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let expected = vec![
            vec!['1', '3', '7', '9', '2', '6', '4', '8', '5'],
            vec!['9', '6', '4', '5', '8', '7', '2', '3', '1'],
            vec!['8', '2', '5', '3', '4', '1', '9', '6', '7'],
            vec!['2', '4', '1', '8', '9', '5', '6', '7', '3'],
            vec!['6', '7', '3', '4', '1', '2', '5', '9', '8'],
            vec!['5', '8', '9', '6', '7', '3', '1', '4', '2'],
            vec!['7', '5', '8', '1', '6', '4', '3', '2', '9'],
            vec!['3', '9', '6', '2', '5', '8', '7', '1', '4'],
            vec!['4', '1', '2', '7', '3', '9', '8', '5', '6'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn discussion_hardest_ever_sudoku_example() {
        let mut input = vec![
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '6', '.', '.', '.', '.', '.'],
            vec!['.', '7', '.', '.', '9', '.', '2', '.', '.'],
            vec!['.', '5', '.', '.', '.', '7', '.', '.', '.'],
            vec!['.', '.', '.', '.', '4', '5', '7', '.', '.'],
            vec!['.', '.', '.', '1', '.', '.', '.', '3', '.'],
            vec!['.', '.', '1', '.', '.', '.', '.', '6', '8'],
            vec!['.', '.', '8', '5', '.', '.', '.', '1', '.'],
            vec!['.', '9', '.', '.', '.', '.', '4', '.', '.'],
        ];
        let expected = vec![
            vec!['8', '1', '2', '7', '5', '3', '6', '4', '9'],
            vec!['9', '4', '3', '6', '8', '2', '1', '7', '5'],
            vec!['6', '7', '5', '4', '9', '1', '2', '8', '3'],
            vec!['1', '5', '4', '2', '3', '7', '8', '9', '6'],
            vec!['3', '6', '9', '8', '4', '5', '7', '2', '1'],
            vec!['2', '8', '7', '1', '6', '9', '5', '3', '4'],
            vec!['5', '2', '1', '9', '7', '4', '3', '6', '8'],
            vec!['4', '3', '8', '5', '2', '6', '9', '1', '7'],
            vec!['7', '9', '6', '3', '1', '8', '4', '5', '2'],
        ];
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }
}
