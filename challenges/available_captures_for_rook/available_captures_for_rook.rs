// https://leetcode.com/problems/available-captures-for-rook/

pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (x0, y0) = {
            let mut x = 0;
            let mut y = 0;
            while x < 8 {
                y = 0;
                while y < 8 {
                    if board[x][y] == 'R' {
                        break;
                    }
                    y += 1;
                }
                if y < 8 {
                    break;
                }
                x += 1;
            }
            (x, y)
        };
        let mut count = 0;
        for x in (0..x0).rev() {
            match board[x as usize][y0 as usize] {
                'p' => {
                    count += 1;
                    break;
                }
                'B' => break,
                _ => {}
            }
        }
        for x in (x0 + 1)..8 {
            match board[x as usize][y0 as usize] {
                'p' => {
                    count += 1;
                    break;
                }
                'B' => break,
                _ => {}
            }
        }
        for y in (0..y0).rev() {
            match board[x0 as usize][y as usize] {
                'p' => {
                    count += 1;
                    break;
                }
                'B' => break,
                _ => {}
            }
        }
        for y in (y0 + 1)..8 {
            match board[x0 as usize][y as usize] {
                'p' => {
                    count += 1;
                    break;
                }
                'B' => break,
                _ => {}
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(board: [[&str; 8]; 8], expected: i32) {
        assert!(board.len() == 8);
        for row in board {
            assert!(row.len() == 8);
            for cell in row {
                assert!(cell.len() == 1);
            }
        }
        assert_eq!(
            Solution::num_rook_captures(
                board
                    .into_iter()
                    .map(|x| x.into_iter().map(|x| x.chars().next().unwrap()).collect())
                    .collect()
            ),
            expected
        );
    }

    fn test_concise(board: [&str; 8], expected: i32) {
        assert!(board.len() == 8);
        for row in board {
            assert!(row.len() == 8);
        }
        assert_eq!(
            Solution::num_rook_captures(board.into_iter().map(|x| x.chars().collect()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            [
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", "R", ".", ".", ".", "p"],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
            ],
            3,
        )
    }

    #[test]
    fn ex2() {
        test(
            [
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", "p", "p", "p", "p", "p", ".", "."],
                [".", "p", "p", "B", "p", "p", ".", "."],
                [".", "p", "B", "R", "B", "p", ".", "."],
                [".", "p", "p", "B", "p", "p", ".", "."],
                [".", "p", "p", "p", "p", "p", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
            ],
            0,
        )
    }

    #[test]
    fn ex3() {
        test(
            [
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                ["p", "p", ".", "R", ".", "p", "B", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "B", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
            ],
            3,
        )
    }

    #[test]
    fn myex0() {
        test_concise(
            [
                "........", "........", ".R......", "........", "........", "........", "........",
                "........",
            ],
            0,
        )
    }
}
