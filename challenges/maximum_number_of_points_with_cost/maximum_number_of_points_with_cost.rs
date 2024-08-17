// https://leetcode.com/problems/maximum-number-of-points-with-cost/

pub struct Solution;

// Full-rank DP sol'n (Inefficient memory-wise, but correct)
// impl Solution {
//     pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
//         let mut dp = vec![vec![0i64; points[0].len()]; points.len()];
//         for i in 0..points[0].len() {
//             dp[0][i] = points[0][i] as i64;
//         }
//         for i in 1..points.len() {
//             let mut left = vec![0; points[0].len()];
//             let mut right = vec![0; points[0].len()];
//             left[0] = dp[i - 1][0];
//             for j in 1..points[i].len() {
//                 left[j] = std::cmp::max(left[j - 1] - 1, dp[i - 1][j]);
//             }
//             right[m - 1] = dp[i - 1][m - 1];
//             for j in (0..points[i].len() - 1).rev() {
//                 right[j] = std::cmp::max(right[j + 1] - 1, dp[i - 1][j]);
//             }
//             for j in 0..points[i].len() {
//                 dp[i][j] = std::cmp::max(left[j], right[j]) + points[i][j] as i64;
//             }
//         }
//         *(dp[n - 1].iter().max().unwrap())
//     }
// }

// Allocation-Optimized sol'n
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let mut points_outer_iter = points.into_iter();
        let points_0 = points_outer_iter.next().unwrap();
        let mut alloc = std::vec::Vec::with_capacity(points_0.len() * 2);
        unsafe {
            alloc.set_len(points_0.len() * 2); // I know what I'm doing lol
        }
        // "dp" is the first half of the alloc
        // "right" is the second half of the alloc
        let (dp, right) = alloc.split_at_mut(points_0.len());
        let mut points_0 = points_0.into_iter();
        dp.fill_with(|| points_0.next().unwrap() as i64);
        for p in points_outer_iter {
            right[dp.len() - 1] = dp[dp.len() - 1];
            for j in (0..dp.len() - 1).rev() {
                right[j] = std::cmp::max(right[j + 1] - 1, dp[j]);
            }
            let mut left_j = 0i64;
            for j in 0..dp.len() {
                left_j = std::cmp::max(left_j - 1, dp[j]);
                dp[j] = std::cmp::max(left_j, right[j]) + p[j] as i64;
            }
        }
        let dp_len = dp.len();
        alloc.truncate(dp_len);
        alloc.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(points: &[&[i32]], output: i64) {
        assert!(points.len() >= 1);
        assert!(points.len() <= 100_000);
        for &p in points {
            assert!(p.len() >= 1);
            assert!(p.len() <= 100_000);
            for &v in p {
                assert!(v >= 0);
                assert!(v <= 100_000);
            }
        }
        let input = points.iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::max_points(input), output);
    }

    #[test]
    fn ex1() {
        test(&[&[1, 2, 3], &[1, 5, 1], &[3, 1, 1]], 9);
    }

    #[test]
    fn ex2() {
        test(&[&[1, 5], &[2, 3], &[4, 2]], 11);
    }
}
