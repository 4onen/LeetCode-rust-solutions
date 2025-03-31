// https://leetcode.com/problems/put-marbles-in-bags/

pub struct Solution;

// Simply collect and sort partition costs
// We can ignore weights[0] and weights[n-1]
// because they are always part of the partition cost.
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut part_cost = weights
            .windows(2)
            .map(|w| w[0] + w[1])
            .collect::<Vec<i32>>();
        part_cost.sort_unstable();
        let min_cost: i64 = part_cost
            .iter()
            .take(k as usize - 1)
            .map(|&c| c as i64)
            .sum();
        let max_cost: i64 = part_cost
            .iter()
            .rev()
            .take(k as usize - 1)
            .map(|&c| c as i64)
            .sum();
        max_cost - min_cost
    }
}

// Optimization: select_nth_unstable (FAILED - spare +1 on rare path of second select_nth_unstable)
// impl Solution {
//     pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
//         let mut part_cost = weights
//             .windows(2)
//             .map(|w| w[0] + w[1])
//             .collect::<Vec<i32>>();
//         if part_cost.len() <= k as usize {
//             return 0;
//         }
//         let (_, _, right) = part_cost.select_nth_unstable(k as usize - 1);
//         if right.len() > k as usize + 1 {
//             right.select_nth_unstable(right.len() - k as usize + 1);
//         }
//         let min_cost: i64 = part_cost
//             .iter()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         let max_cost: i64 = part_cost
//             .iter()
//             .rev()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         max_cost - min_cost
//     }
// }

// Optimization: select_nth_unstable (FAILED - Doesn't work consistently if min and max overlap)
// impl Solution {
//     pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
//         let mut part_cost = weights
//             .windows(2)
//             .map(|w| w[0] + w[1])
//             .collect::<Vec<i32>>();
//         if part_cost.len() <= k as usize {
//             return 0;
//         }
//         let (_, _, right) = part_cost.select_nth_unstable(k as usize - 1);
//         if right.len() > k as usize + 1 {
//             right.select_nth_unstable(right.len() - k as usize);
//         }
//         let min_cost: i64 = part_cost
//             .iter()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         let max_cost: i64 = part_cost
//             .iter()
//             .rev()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         max_cost - min_cost
//     }
// }

// Optimization: Make algorithm constant extra space (in-place weights filling) (FAILED - Off-by-one in the new early-exit condition at the top)
// impl Solution {
//     pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
//         if weights.len() <= k as usize + 1 {
//             return 0;
//         }
//         let mut part_cost = weights;
//         for i in 0..part_cost.len() - 1 {
//             part_cost[i] += part_cost[i + 1];
//         }
//         part_cost.pop();
//         part_cost.sort_unstable();
//         let min_cost: i64 = part_cost
//             .iter()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         let max_cost: i64 = part_cost
//             .iter()
//             .rev()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         max_cost - min_cost
//     }
// }

// Optimization: Make algorithm constant extra space (in-place weights filling) (WORSE PERF)
// impl Solution {
//     pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
//         if weights.len() <= k as usize {
//             return 0;
//         }
//         let mut part_cost = weights;
//         for i in 0..part_cost.len() - 1 {
//             part_cost[i] += part_cost[i + 1];
//         }
//         part_cost.pop();
//         part_cost.sort_unstable();
//         let min_cost: i64 = part_cost
//             .iter()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         let max_cost: i64 = part_cost
//             .iter()
//             .rev()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         max_cost - min_cost
//     }
// }

// Optimization: select_nth_unstable (first one, then the other)
// impl Solution {
//     pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
//         if weights.len() <= k as usize {
//             return 0;
//         }
//         let mut part_cost = weights
//             .windows(2)
//             .map(|w| w[0] + w[1])
//             .collect::<Vec<i32>>();
//         part_cost.select_nth_unstable(k as usize - 1);
//         let min_cost: i64 = part_cost
//             .iter()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         let n = part_cost.len() - k as usize;
//         part_cost.select_nth_unstable(n);
//         let max_cost: i64 = part_cost
//             .iter()
//             .rev()
//             .take(k as usize - 1)
//             .map(|&c| c as i64)
//             .sum();
//         max_cost - min_cost
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(weights: &[i32], k: i32, expected: i64) {
        assert!(k >= 1);
        assert!(k as usize <= weights.len());
        assert!(weights.len() <= 100_000);
        for &weight in weights {
            assert!(weight >= 1);
            assert!(weight <= 1_000_000_000);
        }
        assert_eq!(Solution::put_marbles(weights.to_vec(), k as i32), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 3, 5, 1], 2, 4)
    }

    #[test]
    fn ex2() {
        test(&[1, 3], 2, 0)
    }

    #[test]
    fn discussion_case1() {
        test(&[100, 300, 50, 1, 1], 2, 398)
    }

    #[test]
    fn discussion_case2() {
        test(&[1000000000, 1000000000, 1000000000, 1000000000], 3, 0)
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 4, 2, 5, 2], 3, 3)
    }

    #[test]
    fn discussion_case4() {
        test(&[24, 16, 62, 27, 8, 3, 70, 55, 13, 34, 9, 29, 10], 11, 168)
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                68, 65, 5, 74, 12, 67, 10, 55, 27, 38, 69, 54, 62, 50, 30, 3, 1, 24, 39, 65, 73,
                33, 43, 9, 64,
            ],
            9,
            562,
        )
    }

    #[test]
    fn failing_case1() {
        test(&[25, 74, 16, 51, 12, 48, 15, 5], 1, 0)
    }

    #[test]
    fn failing_case2() {
        test(
            &[
                45, 56, 24, 8, 65, 60, 6, 13, 51, 26, 34, 46, 61, 73, 22, 27, 8, 21, 21, 44,
            ],
            17,
            286,
        )
    }

    #[test]
    fn failing_case3() {
        test(
            &[
                46, 37, 46, 17, 40, 50, 54, 11, 1, 25, 43, 21, 31, 29, 58, 49, 73, 54, 5, 52, 73,
                54, 6, 22, 58, 9, 34, 21, 58, 68, 63,
            ],
            30,
            119,
        )
    }

    #[test]
    fn myex1() {
        test(&[1], 1, 0)
    }

    #[test]
    fn myex1000000000() {
        test(&[1_000_000_000; 100_000], 100_000, 0)
    }
}
