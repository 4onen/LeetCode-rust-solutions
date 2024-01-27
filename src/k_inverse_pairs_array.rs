// https://leetcode.com/problems/k-inverse-pairs-array/

pub struct Solution;

// Full-rank DP sol'n
// impl Solution {
//     pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
//         if n < 1 || k < 0 {
//             return 0;
//         }
//         let n = n as usize;
//         let k = k as usize;
//         const MODULO: u32 = 1_000_000_007;
//         let mut dp = vec![vec![0u32; k + 1]; n + 1];
//         dp[0][0] = 1;
//         dp[1][0] = 1;
//         for i in 2..=n {
//             dp[i][0] = 1;
//             for j in 1..=k {
//                 let partial_sum_before = if j < i { 0 } else { MODULO - dp[i - 1][j - i] };
//                 dp[i][j] = (dp[i][j - 1] + dp[i - 1][j] + partial_sum_before) % MODULO;
//             }
//         }
//         dp[n][k] as i32
//     }
// }

// Two-row DP sol'n
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        if n < 1 || k < 0 {
            return 0;
        }
        let n = n as usize;
        let k = k as usize;
        const MODULO: u32 = 1_000_000_007;
        let mut prev = vec![0u32; k + 1];
        let mut curr = vec![0u32; k + 1];
        prev[0] = 1;
        curr[0] = 1;
        for i in 2..=n {
            for j in 1..=k {
                let partial_sum_before = if j < i { 0 } else { MODULO - prev[j - i] };
                curr[j] = (curr[j - 1] + prev[j] + partial_sum_before) % MODULO;
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        prev[k] as i32
    }
}

#[allow(dead_code)]
const LUT: &[[u32; 20]] = &[
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 3, 5, 6, 5, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [
        1, 4, 9, 15, 20, 22, 20, 15, 9, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 5, 14, 29, 49, 71, 90, 101, 101, 90, 71, 49, 29, 14, 5, 1, 0, 0, 0, 0,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
        // 1, 2, 3
        // is the only permutation with 0 inverse pairs.
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
        // 1, 3, 2
        // 2, 1, 3
        // are the only permutations with 1 inverse pair.
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::k_inverse_pairs(4, 1), 3);
        // 1, 2, 4, 3
        // 2, 1, 3, 4
        // 1, 3, 2, 4
        // are the only permutations with 1 inverse pair.
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::k_inverse_pairs(4, 2), 5);
        // 2, 1, 4, 3
        // 1, 4, 2, 3
        // 1, 3, 4, 2
        // 3, 1, 2, 4
        // 2, 3, 1, 4
        // are the only permutations with 2 inverse pairs.
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::k_inverse_pairs(4, 3), 6);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::k_inverse_pairs(3, 2), 2);
        // 2, 3, 1
        // 3, 1, 2
        // are the only permutations with 2 inverse pairs.
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::k_inverse_pairs(3, 3), 1);
        // 3, 2, 1
        // is the only permutation with 3 inverse pairs.
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::k_inverse_pairs(3, 4), 0);
        // There are no permutations with 4 inverse pairs.
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::k_inverse_pairs(2, 0), 1);
        // 1, 2
        // is the only permutation with 0 inverse pairs.
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::k_inverse_pairs(2, 1), 1);
        // 2, 1
        // is the only permutation with 1 inverse pair.
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::k_inverse_pairs(2, 2), 0);
        // There are no permutations with 2 inverse pairs.
    }

    #[test]
    fn myex9() {
        assert_eq!(Solution::k_inverse_pairs(1, 0), 1);
        // 1
        // is the only permutation with 0 inverse pairs.
    }

    #[test]
    fn myex10() {
        assert_eq!(Solution::k_inverse_pairs(1, 1), 0);
        // There are no permutations with 1 inverse pair.
    }

    #[test]
    fn myex11() {
        assert_eq!(Solution::k_inverse_pairs(4, 4), 5);
    }

    #[test]
    fn failing_case1() {
        assert_eq!(Solution::k_inverse_pairs(1000, 1000), 663677020);
    }
}

// All permutations of 1,2,3:
// 1, 2, 3 -> 0 inverse pairs
// 1, 3, 2 -> 1
// 2, 1, 3 -> 1
// 2, 3, 1 -> 2
// 3, 1, 2 -> 2
// 3, 2, 1 -> 3

// All permutations of 1,2,3,4:
// 1, 2, 3, 4 -> 0 inverse pairs
// 1, 2, 4, 3 -> 1
// 1, 3, 2, 4 -> 1
// 1, 3, 4, 2 -> 2
// 1, 4, 2, 3 -> 2
// 1, 4, 3, 2 -> 3
// 2, 1, 3, 4 -> 1
// 2, 1, 4, 3 -> 2
// 2, 3, 1, 4 -> 2
// 2, 3, 4, 1 -> 3
// 2, 4, 1, 3 -> 3
// 2, 4, 3, 1 -> 4
// 3, 1, 2, 4 -> 2
// 3, 1, 4, 2 -> 3
// 3, 2, 1, 4 -> 3
// 3, 2, 4, 1 -> 4
// 3, 4, 1, 2 -> 4
// 3, 4, 2, 1 -> 5
// 4, 1, 2, 3 -> 3
// 4, 1, 3, 2 -> 4
// 4, 2, 1, 3 -> 4
// 4, 2, 3, 1 -> 5
// 4, 3, 1, 2 -> 5
// 4, 3, 2, 1 -> 6

// All permutations of 1,2,3,4,5
// 1,2,3,4,5 -> 0
// 1,2,3,5,4 -> 1
// 1,2,4,3,5 -> 1
// 1,2,4,5,3 -> 2
// 1,2,5,3,4 -> 2
// 1,2,5,4,3 -> 3
// 1,3,2,4,5 -> 1
// 1,3,2,5,4 -> 2
// 1,3,4,2,5 -> 2
// 1,3,4,5,2 -> 3
// 1,3,5,2,4 -> 3
// 1,3,5,4,2 -> 4
// 1,4,2,3,5 -> 2
// 1,4,2,5,3 -> 3
// 1,4,3,2,5 -> 3
// 1,4,3,5,2 -> 4
// 1,4,5,2,3 -> 4
// 1,4,5,3,2 -> 5
// 1,5,2,3,4 -> 3
// 1,5,2,4,3 -> 4
// 1,5,3,2,4 -> 4
// 1,5,3,4,2 -> 5
// 1,5,4,2,3 -> 5
// 1,5,4,3,2 -> 6
// 2,1,3,4,5 -> 1
// 2,1,3,5,4 -> 2
// 2,1,4,3,5 -> 2
// 2,1,4,5,3 -> 3
// 2,1,5,3,4 -> 3
// 2,1,5,4,3 -> 4
// 2,3,1,4,5 -> 2
// 2,3,1,5,4 -> 3
// 2,3,4,1,5 -> 3
// 2,3,4,5,1 -> 4
// 2,3,5,1,4 -> 4
// 2,3,5,4,1 -> 5
// 2,4,1,3,5 -> 3
// 2,4,1,5,3 -> 4
// 2,4,3,1,5 -> 4
// 2,4,3,5,1 -> 5
// 2,4,5,1,3 -> 5
// 2,4,5,3,1 -> 6
// 2,5,1,3,4 -> 4
// 2,5,1,4,3 -> 5
// 2,5,3,1,4 -> 5
// 2,5,3,4,1 -> 6
// 2,5,4,1,3 -> 6
// 2,5,4,3,1 -> 7
// 3,1,2,4,5 -> 2
// 3,1,2,5,4 -> 3
// 3,1,4,2,5 -> 3
// 3,1,4,5,2 -> 4
// 3,1,5,2,4 -> 4
// 3,1,5,4,2 -> 5
// 3,2,1,4,5 -> 3
// 3,2,1,5,4 -> 4
// 3,2,4,1,5 -> 4
// 3,2,4,5,1 -> 5
// 3,2,5,1,4 -> 5
// 3,2,5,4,1 -> 6
// 3,4,1,2,5 -> 4
// 3,4,1,5,2 -> 5
// 3,4,2,1,5 -> 5
// 3,4,2,5,1 -> 6
// 3,4,5,1,2 -> 6
// 3,4,5,2,1 -> 7
// 3,5,1,2,4 -> 5
// 3,5,1,4,2 -> 6
// 3,5,2,1,4 -> 6
// 3,5,2,4,1 -> 7
// 3,5,4,1,2 -> 7
// 3,5,4,2,1 -> 8
// 4,1,2,3,5 -> 3
// 4,1,2,5,3 -> 4
// 4,1,3,2,5 -> 4
// 4,1,3,5,2 -> 5
// 4,1,5,2,3 -> 5
// 4,1,5,3,2 -> 6
// 4,2,1,3,5 -> 4
// 4,2,1,5,3 -> 5
// 4,2,3,1,5 -> 5
// 4,2,3,5,1 -> 6
// 4,2,5,1,3 -> 6
// 4,2,5,3,1 -> 7
// 4,3,1,2,5 -> 5
// 4,3,1,5,2 -> 6
// 4,3,2,1,5 -> 6
// 4,3,2,5,1 -> 7
// 4,3,5,1,2 -> 7
// 4,3,5,2,1 -> 8
// 4,5,1,2,3 -> 6
// 4,5,1,3,2 -> 7
// 4,5,2,1,3 -> 7
// 4,5,2,3,1 -> 8
// 4,5,3,1,2 -> 8
// 4,5,3,2,1 -> 9
// 5,1,2,3,4 -> 4
// 5,1,2,4,3 -> 5
// 5,1,3,2,4 -> 5
// 5,1,3,4,2 -> 6
// 5,1,4,2,3 -> 6
// 5,1,4,3,2 -> 7
// 5,2,1,3,4 -> 5
// 5,2,1,4,3 -> 6
// 5,2,3,1,4 -> 6
// 5,2,3,4,1 -> 7
// 5,2,4,1,3 -> 7
// 5,2,4,3,1 -> 8
// 5,3,1,2,4 -> 6
// 5,3,1,4,2 -> 7
// 5,3,2,1,4 -> 7
// 5,3,2,4,1 -> 8
// 5,3,4,1,2 -> 8
// 5,3,4,2,1 -> 9
// 5,4,1,2,3 -> 7
// 5,4,1,3,2 -> 8
// 5,4,2,1,3 -> 8
// 5,4,2,3,1 -> 9
// 5,4,3,1,2 -> 9
// 5,4,3,2,1 -> 10
