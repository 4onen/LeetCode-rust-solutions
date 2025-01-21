// https://leetcode.com/problems/grid-game/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
//         fn to_prefix(arr: &[i32]) -> Vec<i64> {
//             arr.iter().scan(0,|acc, &x| -> Option<i64> {
//                 *acc += x as i64;
//                 Some(*acc)
//             }).collect()
//         }
//         let (row0, row1) = {
//             let mut grid = grid.into_iter();
//             (grid.next().unwrap(), grid.next().unwrap())
//         };
//         let row0_prefix = to_prefix(&row0);
//         let row1_prefix = to_prefix(&row1);
//         std::mem::drop(row0);
//         std::mem::drop(row1);
//         assert_eq!(row0_prefix.len(),row1_prefix.len());
//         let row0_sum = row0_prefix.last().unwrap();
//         let mut res = i64::MAX;
//         for i in 0..row0_prefix.len() {
//             let top = row0_sum - row0_prefix[i];
//             let bot = if i > 0 { row1_prefix[i-1] } else { 0 };
//             let optimal_second = std::cmp::max(top, bot);
//             if optimal_second < res {
//                 res = optimal_second;
//             }
//         }
//         res
//     }
// }

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let (row0, row1) = {
            let mut grid = grid.into_iter();
            (grid.next().unwrap(), grid.next().unwrap())
        };
        let n = row0.len() as u16;
        let mut workspace = vec![0i64; 2 * n as usize];
        let (row0_prefix, row1_prefix) = workspace.split_at_mut(n as usize);
        row0_prefix[0] = row0[0] as i64;
        row1_prefix[0] = row1[0] as i64;
        for i in 1..n {
            row0_prefix[i as usize] = row0_prefix[i as usize - 1] + row0[i as usize] as i64;
            row1_prefix[i as usize] = row1_prefix[i as usize - 1] + row1[i as usize] as i64;
        }
        let row0_sum = row0_prefix.last().unwrap();
        let mut res = i64::MAX;
        for i in 0..row0_prefix.len() {
            let top = row0_sum - row0_prefix[i];
            let bot = if i > 0 { row1_prefix[i - 1] } else { 0 };
            let optimal_second = std::cmp::max(top, bot);
            if optimal_second < res {
                res = optimal_second;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]; 2], expected: i64) {
        for &row in grid {
            assert!(row.len() >= 1);
            assert!(row.len() <= 50_000);
            assert_eq!(row.len(), grid[0].len());
            for &el in row {
                assert!(el >= 1);
                assert!(el <= 100_000);
            }
        }
        assert_eq!(
            Solution::grid_game(grid.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[2, 5, 4], &[1, 5, 1]], 4)
    }

    #[test]
    fn ex2() {
        test(&[&[3, 3, 1], &[8, 5, 2]], 4)
    }

    #[test]
    fn ex3() {
        test(&[&[1, 3, 1, 15], &[1, 3, 3, 1]], 7)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                &[20, 03, 20, 17, 02, 12, 15, 17, 04, 15],
                &[20, 10, 13, 14, 15, 05, 02, 03, 14, 03],
            ],
            63,
        )
    }

    #[test]
    fn discussion_case2() {
        test(&[&[3, 3, 21], &[18, 15, 2]], 21)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[&[100_000; 50_000], &[1; 50_000]], 49_999)
    }
}
