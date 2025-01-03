// https://leetcode.com/problems/number-of-ways-to-split-array/

pub struct Solution;

// Simple prefix list
// impl Solution {
//     pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
//         let prefix_sum = nums
//             .into_iter()
//             .scan(0, |acc, num| {
//                 *acc += num as i64;
//                 Some(*acc)
//             })
//             .collect::<Vec<_>>();
//         let tot = *prefix_sum.last().unwrap();
//         let mut count = 0;
//         for i in 0..prefix_sum.len() - 1 {
//             count += (2 * prefix_sum[i] >= tot) as i32;
//         }
//         count
//     }
// }

// Linear time, constant space
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let tot = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut so_far = 0;
        let mut count = 0;
        let iter_end = nums.len() - 1;
        for num in nums.into_iter().take(iter_end) {
            so_far += num as i64;
            count += (2 * so_far >= tot) as i32;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 2);
        assert!(nums.len() <= 100_000);
        for &num in nums {
            assert!(num >= -100_000);
            assert!(num <= 100_000);
        }
        assert_eq!(Solution::ways_to_split_array(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[10, 4, -8, 7], 2)
    }

    #[test]
    fn ex2() {
        test(&[2, 3, 1, 0], 2)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[-100_000; 100_000], 50_000);
    }

    #[test]
    fn my_extreme_ex2() {
        test(&[100_000; 100_000], 50_000);
    }
}
