// https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
//         type Idx = u8;
//         type Cost = u8;
//         assert!(grid.len() <= 100);
//         let n = grid.len() as Idx;
//         let m = grid[0].len() as Idx;
//         let mut cost_grid = vec![vec![Cost::MAX;grid[0].len()];grid.len()];
//         cost_grid[0][0] = 0;
//         let mut to_process = std::collections::VecDeque::new();
//         to_process.push_back((0 as Idx,0 as Idx));
//         while let Some((row, col)) = to_process.pop_front() {
//             let my_cost = cost_grid[row as usize][col as usize];
//             let my_sign = grid[row as usize][col as usize];
//             if col < m-1 {
//                 let (their_row, their_col) = (row, col + 1);
//                 let their_cost = cost_grid[their_row as usize][their_col as usize];
//                 let new_their_cost = my_cost.saturating_add((my_sign != 1) as Cost);
//                 if new_their_cost < their_cost {
//                     cost_grid[their_row as usize][their_col as usize] = new_their_cost;
//                     to_process.push_back((their_row, their_col));
//                 }
//             }
//             if col > 0 {
//                 let (their_row, their_col) = (row, col - 1);
//                 let their_cost = cost_grid[their_row as usize][their_col as usize];
//                 let new_their_cost = my_cost.saturating_add((my_sign != 2) as Cost);
//                 if new_their_cost < their_cost {
//                     cost_grid[their_row as usize][their_col as usize] = new_their_cost;
//                     to_process.push_back((their_row, their_col));
//                 }
//             }
//             if row < n-1 {
//                 let (their_row, their_col) = (row+1, col);
//                 let their_cost = cost_grid[their_row as usize][their_col as usize];
//                 let new_their_cost = my_cost.saturating_add((my_sign != 3) as Cost);
//                 if new_their_cost < their_cost {
//                     cost_grid[their_row as usize][their_col as usize] = new_their_cost;
//                     to_process.push_back((their_row, their_col));
//                 }
//             }
//             if row > 0 {
//                 let (their_row, their_col) = (row - 1, col);
//                 let their_cost = cost_grid[their_row as usize][their_col as usize];
//                 let new_their_cost = my_cost.saturating_add((my_sign != 4) as Cost);
//                 if new_their_cost < their_cost {
//                     cost_grid[their_row as usize][their_col as usize] = new_their_cost;
//                     to_process.push_back((their_row, their_col));
//                 }
//             }
//         }
//         *cost_grid.last().unwrap().last().unwrap() as i32
//     }
// }

// Precalc dirs, process zero-cost paths first
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        type Idx = i8;
        type Cost = u8;
        const DIRS: [(Idx,Idx);4] = [
            (0,1),
            (0,-1),
            (1,0),
            (-1,0),
        ];
        assert!(grid.len() <= 100);
        let n = grid.len() as Idx;
        let m = grid[0].len() as Idx;
        let mut cost_grid = vec![vec![Cost::MAX;grid[0].len()];grid.len()];
        cost_grid[0][0] = 0;
        let mut to_process = std::collections::VecDeque::new();
        to_process.push_back((0 as Idx,0 as Idx));
        while let Some((row, col)) = to_process.pop_front() {
            let my_cost = cost_grid[row as usize][col as usize];
            let my_dir = grid[row as usize][col as usize];
            for i in 0..DIRS.len() as u8 {
                let (drow,dcol) = DIRS[i as usize];
                let (their_row, their_col) = (row + drow, col + dcol);
                if their_row < 0 || their_row >= n || their_col < 0 || their_col >= m {
                    continue;
                }
                let against_sign = (i+1) as i32 != my_dir;
                let their_new_cost = my_cost.saturating_add(against_sign as Cost);
                if their_new_cost >= cost_grid[their_row as usize][their_col as usize] {
                    continue;
                }
                cost_grid[their_row as usize][their_col as usize] = their_new_cost;
                if against_sign {
                    to_process.push_back((their_row, their_col));
                } else {
                    to_process.push_front((their_row, their_col));
                }
            }
        }
        *cost_grid.last().unwrap().last().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 100);
        assert!(grid[0].len() >= 1);
        assert!(grid[0].len() <= 100);
        for &row in grid {
            assert_eq!(row.len(), grid[0].len());
            for &el in row {
                assert!(el >= 1);
                assert!(el <= 4);
            }
        }
        assert_eq!(Solution::min_cost(grid.iter().map(|x| x.to_vec()).collect()), expected);
    }

    #[test]
    fn ex1() {
        test(&[&[1,1,1,1],&[2,2,2,2],&[1,1,1,1],&[2,2,2,2]], 3)
    }

    #[test]
    fn ex2() {
        test(&[&[1,1,3],&[3,2,2],&[1,1,4]], 0)
    }

    #[test]
    fn ex3() {
        test(&[&[1,2],&[4,3]], 1)
    }

    #[test]
    fn my_extreme_ex1() {
        let row: &[i32] = &[1i32;100];
        test(&[row;100], 99)
    }

    #[test]
    fn my_extreme_ex2() {
        let row: &[i32] = &[1i32;50];
        test(&[row;100], 99)
    }

    #[test]
    fn my_extreme_ex3() {
        let row: &[i32] = &[1i32;100];
        test(&[row;50], 49)
    }

    #[test]
    fn discussion_case1() {
        test(&[&[1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2]], 9)
    }

    #[test]
    fn discussion_case2() {
        test(&[&[1,2,1,2,1,2,1,2,1,2,1,2],&[2,1,2,1,2,1,2,1,2,1,2,1],&[1,2,1,2,1,2,1,2,1,2,1,2],&[2,1,2,1,2,1,2,1,2,1,2,1],&[1,2,1,2,1,2,1,2,1,2,1,2],&[2,1,2,1,2,1,2,1,2,1,2,1],&[1,2,1,2,1,2,1,2,1,2,1,2],&[2,1,2,1,2,1,2,1,2,1,2,1],&[1,2,1,2,1,2,1,2,1,2,1,2],&[2,1,2,1,2,1,2,1,2,1,2,1]],10)
    }

    #[test]
    fn discussion_case3() {
        test(&[&[1,2,3,4,1,2,3,4,1,2],&[4,1,2,3,4,1,2,3,4,1],&[2,3,4,1,2,3,4,1,2,3],&[3,4,1,2,3,4,1,2,3,4],&[4,1,2,3,4,1,2,3,4,1],&[1,2,3,4,1,2,3,4,1,2],&[2,3,4,1,2,3,4,1,2,3],&[3,4,1,2,3,4,1,2,3,4],&[4,1,2,3,4,1,2,3,4,1],&[1,2,3,4,1,2,3,4,1,2]],8)
    }

    #[test]
    fn discussion_case4() {
        test(&[&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],&[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],&[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]],17)
    }

    #[test]
    fn discussion_case5() {
        test(&[&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2],&[1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1],&[2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2]],18)
    }
}
