// https://leetcode.com/problems/maximum-subarray/

pub struct Solution;

// Kadane's algorithm
// impl Solution {
//     pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//         let mut max_ending_here = nums[0];
//         let mut max_so_far = nums[0];
//         for v in nums.into_iter().skip(1) {
//             max_ending_here = std::cmp::max(v, max_ending_here + v);
//             max_so_far = std::cmp::max(max_so_far, max_ending_here);
//         }
//         max_so_far
//     }
// }

// Kadane's algorithm v2
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_ending_here = 0;
        let mut max_so_far = std::i32::MIN;
        for v in nums.into_iter() {
            max_ending_here += v;
            if max_ending_here > max_so_far {
                max_so_far = max_ending_here;
            }
            if max_ending_here < 0 {
                max_ending_here = 0;
            }
        }
        max_so_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn myex1() {
        let inputs = (0..100_000)
            .into_iter()
            .map(|v| v % 20_001 - 10_000)
            .collect::<Vec<_>>();
        assert_eq!(Solution::max_sub_array(inputs), 50_005_000);
    }
}
