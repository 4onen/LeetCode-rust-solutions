// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_difference(mut nums: Vec<i32>) -> i32 {
//         if nums.len() <= 4 {
//             return 0;
//         }
//         nums.sort_unstable();
//         let mut res = i32::MAX;
//         let n = nums.len();
//         for i in 0..4 {
//             res = std::cmp::min(res,nums[n + i - 4] - nums[i]);
//         }
//         res
//     }
// }

// Unroll the loop by hand
// impl Solution {
//     pub fn min_difference(mut nums: Vec<i32>) -> i32 {
//         if nums.len() <= 4 {
//             return 0;
//         }
//         nums.sort_unstable();
//         let n = nums.len();
//         std::cmp::min(
//             std::cmp::min(
//                 nums[n - 4] - nums[0],
//                 nums[n - 3] - nums[1],
//             ),
//             std::cmp::min(
//                 nums[n - 2] - nums[2],
//                 nums[n - 1] - nums[3],
//             ),
//         )
//     }
// }

// Use select_nth_unstable instead of sort_unstable to skip the middle
impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 4 {
            return 0;
        }
        nums.select_nth_unstable(n-4);
        nums[n-4..].sort_unstable();
        nums.select_nth_unstable(3);
        nums[0..4].sort_unstable();
        std::cmp::min(
            nums[n-4] - nums[0],
            std::cmp::min(
                nums[n-3] - nums[1],
                std::cmp::min(
                    nums[n-2] - nums[2],
                    nums[n-1] - nums[3],
                ),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() > 0);
        assert!(nums.len() <= 100_000);
        assert_eq!(Solution::min_difference(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[5,3,2,4], 0);
    }

    #[test]
    fn ex2() {
        test(&[1,5,0,10,14], 1);
    }

    #[test]
    fn ex3() {
        test(&[3,100,20], 0);
    }

    #[test]
    fn myex1() {
        test(&[12,11,10,9,8,7,6,5,4,3,2,1], 8);
    }

    #[test]
    fn myex2() {
        test(&[13,12,11,10,9,8,7,6,5,4,3,2,1], 9);
    }
}
