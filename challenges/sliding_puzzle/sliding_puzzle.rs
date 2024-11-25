// https://leetcode.com/problems/sliding-puzzle/

pub struct Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        type Board = [u8; 6];
        const TARGET: Board = [1, 2, 3, 4, 5, 0];
        let board: Board = {
            let mut new_board = [0; 6];
            for i in 0..=1 {
                for j in 0..3 {
                    new_board[i * 3 + j] = board[i][j] as u8;
                }
            }
            new_board
        };
        if board == TARGET {
            return 0;
        }
        let mut visited = std::collections::HashSet::new();
        visited.insert(board);
        let mut queue = vec![board];
        let mut next_queue = std::vec::Vec::new();
        let mut steps = 0;
        while !queue.is_empty() {
            while !queue.is_empty() {
                let board = queue.pop().unwrap();
                if board == TARGET {
                    return steps;
                }
                let idx: i8 = {
                    let mut idx = 0;
                    while board[idx as usize] != 0 {
                        idx += 1;
                    }
                    assert!(idx < 6);
                    idx
                };
                for di in &[1, 3, -1, -3] {
                    match (idx, di) {
                        (3, -1) | (2, 1) => continue,
                        _ => {}
                    }
                    let slide_from = idx + di;
                    if !(0..6).contains(&slide_from) {
                        continue;
                    }
                    let mut new_board: Board = board;
                    let slid = std::mem::replace(&mut new_board[slide_from as usize], 0);
                    new_board[idx as usize] = slid;
                    if visited.insert(new_board) {
                        next_queue.push(new_board);
                    }
                }
            }
            std::mem::swap(&mut queue, &mut next_queue);
            steps += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(board: [[i32; 3]; 2], expected: i32) {
        assert_eq!(
            Solution::sliding_puzzle(board.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test([[1, 2, 3], [4, 0, 5]], 1)
    }

    #[test]
    fn ex1_0() {
        test([[1, 2, 3], [4, 5, 0]], 0)
    }

    #[test]
    fn ex2() {
        test([[1, 2, 3], [5, 4, 0]], -1)
    }

    #[test]
    fn ex3() {
        test([[4, 1, 2], [5, 0, 3]], 5)
    }

    #[test]
    fn discussion_case1() {
        test([[1, 3, 4], [0, 2, 5]], 14)
    }

    #[test]
    fn discussion_case2() {
        test([[3, 2, 4], [1, 5, 0]], 14)
    }

    #[test]
    fn discussion_case3() {
        test([[2, 4, 1], [5, 3, 0]], 12)
    }

    #[test]
    fn discussion_case4() {
        test([[0, 5, 2], [4, 3, 1]], 15)
    }

    #[test]
    fn discussion_case6() {
        test([[4, 2, 0], [5, 1, 3]], 7)
    }
}
