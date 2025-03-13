// https://leetcode.com/problems/zero-array-transformation-i/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
//         let mut diffs = vec![0; nums.len() + 1];
//         for q in queries {
//             let l = q[0] as usize;
//             let r = q[1] as usize;
//             diffs[l] += 1;
//             diffs[r + 1] -= 1;
//         }
//         let best_possible = {
//             for i in 1..diffs.len() {
//                 diffs[i] += diffs[i - 1];
//             }
//             diffs
//         };
//         std::iter::zip(nums.into_iter(), best_possible.into_iter()).all(|(n, b)| n <= b)
//     }
// }

// Removing redundant pass
// impl Solution {
//     pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
//         let mut diffs = vec![0; nums.len() + 1];
//         for q in queries {
//             let l = q[0] as usize;
//             let r = q[1] as usize;
//             diffs[l] += 1;
//             diffs[r + 1] -= 1;
//         }
//         let mut sum = 0;
//         for (num, diff) in std::iter::zip(nums, diffs) {
//             sum += diff;
//             if num > sum {
//                 return false;
//             }
//         }
//         true
//     }
// }

// Optimizing: Use iter() to avoid deallocs mid-compute
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut diffs = vec![0; nums.len() + 1];
        for q in queries.iter() {
            let l = q[0] as usize;
            let r = q[1] as usize;
            diffs[l] += 1;
            diffs[r + 1] -= 1;
        }
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += diffs[i as usize];
            if nums[i as usize] > sum {
                return false;
            }
        }
        true
    }
}

// Optimizing: Flip signs to optimize heavy comparison
// impl Solution {
//     pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
//         let mut diffs = vec![0; nums.len() + 1];
//         for q in queries.iter() {
//             let l = q[0] as usize;
//             let r = q[1] as usize;
//             diffs[l] -= 1;
//             diffs[r + 1] += 1;
//         }
//         let mut sum = 0;
//         for i in 0..nums.len() {
//             sum += diffs[i as usize];
//             if nums[i as usize] + sum > 0 {
//                 return false;
//             }
//         }
//         true
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], queries: &[[i32; 2]], expected: bool) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 100_000);
        for &n in nums {
            assert!(n <= 500_000);
        }
        for &[l, r] in queries {
            assert!(l <= r);
            assert!((r as usize) < nums.len());
        }
        assert_eq!(
            Solution::is_zero_array(
                nums.iter().map(|&x| x as i32).collect(),
                queries
                    .iter()
                    .map(|&[l, r]| vec![l as i32, r as i32])
                    .collect(),
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 0, 1], &[[0, 2]], true);
    }

    #[test]
    fn ex2() {
        test(&[4, 3, 2, 1], &[[1, 3], [0, 2]], false);
    }

    #[test]
    fn discussion_case1() {
        test(&[2], &[[0, 0], [0, 0], [0, 0]], true);
    }
}
