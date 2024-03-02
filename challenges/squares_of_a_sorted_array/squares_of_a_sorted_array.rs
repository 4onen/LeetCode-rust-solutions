// https://leetcode.com/problems/squares-of-a-sorted-array/

pub struct Solution;

// Braindead sol'n
// impl Solution {
//     pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
//         nums.iter_mut().for_each(|n| *n *= *n);
//         nums.sort_unstable();
//         nums
//     }
// }

// One-pass two-pointer sol'n (out-of-place)
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut left = 0u16;
        let mut right = nums.len() as u16 - 1;
        for i in (0..nums.len()).rev() {
            let left_val = nums[left as usize];
            let right_val = nums[right as usize];
            result[i as usize] = if left_val.abs() > right_val.abs() {
                left += 1;
                left_val * left_val
            } else {
                right -= 1;
                right_val * right_val
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
