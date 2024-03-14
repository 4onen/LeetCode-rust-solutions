// https://leetcode.com/problems/count-number-of-nice-subarrays/

pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        assert!(nums.len() <= 50_000);
        assert!(k >= 1);
        let k = k as u16;
        assert!(k <= nums.len() as u16);
        let mut count = 0;
        let mut sum: usize = 0;
        let mut indices = vec![-1];
        for i in 0..nums.len() as u16 {
            if nums[i as usize] % 2 == 1 {
                sum += 1;
                indices.push(i as i32);
            }
            if let Some(idx) = sum.checked_sub(k as usize) {
                count += indices[idx + 1] - indices[idx as usize];
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }

    #[test]
    fn binary_ex1() {
        let nums = [1, 0, 1, 0, 1];
        let goal = 2;
        let expected = 4;
        assert_eq!(Solution::number_of_subarrays(nums.to_vec(), goal), expected);
    }

    #[test]
    fn binary_ex3() {
        let nums = [0, 0, 0, 0, 0];
        let goal = 1;
        let expected = 0;
        assert_eq!(Solution::number_of_subarrays(nums.to_vec(), goal), expected);
    }

    #[test]
    fn binary_failed_case1() {
        let nums = [0, 0, 0, 0, 1];
        let goal = 2;
        let expected = 0;
        assert_eq!(Solution::number_of_subarrays(nums.to_vec(), goal), expected);
    }

    #[test]
    fn binary_myex1() {
        let nums = [1, 0, 1, 0];
        let goal = 2;
        let expected = 2;
        assert_eq!(Solution::number_of_subarrays(nums.to_vec(), goal), expected);
    }

    #[test]
    fn binary_myex2() {
        let nums = [1, 0, 1];
        let goal = 2;
        let expected = 1;
        assert_eq!(Solution::number_of_subarrays(nums.to_vec(), goal), expected);
    }

    #[test]
    fn oops_all_odds1() {
        let nums = [1, 1, 1, 1, 1];
        let goal = 1;
        let expected = 5;
        assert_eq!(Solution::number_of_subarrays(nums.to_vec(), goal), expected);
    }

    #[test]
    fn oops_all_odds2() {
        let nums = [1, 1, 1, 1, 1];
        let goal = 2;
        let expected = 4;
        assert_eq!(Solution::number_of_subarrays(nums.to_vec(), goal), expected);
    }
}
