// https://leetcode.com/problems/longest-increasing-subsequence/

pub struct Solution;

// O(n^2) solution
// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let mut dp = vec![1; nums.len()];
//         let mut max = 0;
//         for i in 0..nums.len() {
//             for j in 0..i {
//                 if nums[i] > nums[j] {
//                     dp[i] = std::cmp::max(dp[i], dp[j] + 1);
//                 }
//             }
//             max = std::cmp::max(max, dp[i]);
//         }
//         max
//     }
// }

// O(nlogn) solution
// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let mut dp = vec![0; nums.len()];
//         let mut len = 0;
//         for num in nums {
//             let insertion_pos = dp[..len].binary_search(&num).unwrap_or_else(|x| x);
//             dp[insertion_pos] = num;
//             if insertion_pos == len {
//                 len += 1;
//             }
//         }
//         len as i32
//     }
// }

// O(nlogn) solution with optimized first comparison
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut len = 1;
        dp[0] = nums[0];
        for num in nums.into_iter().skip(1) {
            match dp[len - 1].cmp(&num) {
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Less => {
                    dp[len] = num;
                    len += 1;
                }
                std::cmp::Ordering::Greater => match dp[..len].binary_search(&num) {
                    Ok(_) => continue,
                    Err(insertion_pos) => dp[insertion_pos] = num,
                },
            }
        }
        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::length_of_lis(vec![1, 2]), 2);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::length_of_lis(vec![2, 1]), 1);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::length_of_lis(vec![1, 3, 2]), 2);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::length_of_lis(vec![1, 3, 2, 4]), 3);
    }

    #[test]
    fn myex5() {
        assert_eq!(
            Solution::length_of_lis((1..=2500).into_iter().collect()),
            2500,
        );
    }

    #[test]
    fn myex6() {
        assert_eq!(
            Solution::length_of_lis((1..=2500).into_iter().rev().collect()),
            1
        );
    }
}
