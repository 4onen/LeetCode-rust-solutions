// https://leetcode.com/problems/partition-array-for-maximum-sum/description/

pub struct Solution;

// O(k) rotating dp array (2ms)
// impl Solution {
//     pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
//         let k = k as usize;
//         let l = arr.len();
//         let mut dp = vec![0; k];
//         for i in 0..l {
//             let mut largest_arr = 0;
//             let mut largest_dp = 0;
//             for j in 1..=std::cmp::min(i + 1, k) {
//                 largest_arr = std::cmp::max(largest_arr, arr[i + 1 - j]);
//                 largest_dp = std::cmp::max(largest_dp, dp[k - j] + largest_arr * j as i32);
//             }
//             dp.rotate_left(1);
//             *dp.last_mut().unwrap() = largest_dp;
//         }
//         *dp.last().unwrap()
//     }
// }

// O(n+1) dp array (0ms)
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let l = arr.len();
        let mut dp = vec![0; l + 1];
        for i in 1..=l {
            let mut largest_arr = 0;
            let mut largest_dp = 0;
            for j in 1..=std::cmp::min(i, k) {
                largest_arr = std::cmp::max(largest_arr, arr[i - j]);
                largest_dp = std::cmp::max(largest_dp, dp[i - j] + largest_arr * j as i32);
            }
            dp[i] = largest_dp;
        }
        dp[l]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::max_sum_after_partitioning(vec![1], 1), 1)
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::max_sum_after_partitioning(vec![1, 15], 2), 30)
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::max_sum_after_partitioning(vec![1, 15], 1), 16)
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::max_sum_after_partitioning(vec![1, 15], 3), 30)
    }

    #[test]
    fn my_extrema_ex1() {
        assert_eq!(
            Solution::max_sum_after_partitioning(
                (0..500).into_iter().map(|_| 4_000_000).collect(),
                1
            ),
            4_000_000 * 500
        )
    }
}
