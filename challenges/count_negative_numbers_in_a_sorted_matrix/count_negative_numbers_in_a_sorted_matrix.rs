// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/

pub struct Solution;

// Initial O(nm) sol'n
// impl Solution {
//     pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
//         grid.into_iter()
//             .map(|row| {
//                 let row_len = row.len();
//                 let nonnegative = row.into_iter().take_while(|&x| x >= 0).count();
//                 row_len - nonnegative
//             })
//             .sum::<usize>() as i32
//     }
// }

// O(n+m) sol'n
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut nonnegative = 0;
        for row in grid.into_iter().rev() {
            while nonnegative < row.len() && row[nonnegative as usize] >= 0 {
                nonnegative += 1;
            }
            res += row.len() - nonnegative
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        assert_eq!(
            Solution::count_negatives(grid.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[
                &[4, 3, 2, -1],
                &[3, 2, 1, -1],
                &[1, 1, -1, -2],
                &[-1, -1, -2, -3],
            ],
            8,
        )
    }

    #[test]
    fn ex2() {
        test(&[&[3, 2], &[1, 0]], 0)
    }
}
