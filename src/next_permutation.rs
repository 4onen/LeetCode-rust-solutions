// https://leetcode.com/problems/next-permutation/

pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        match nums.windows(2).rposition(|w| w[0] < w[1]) {
            None => nums.reverse(),
            Some(i) => {
                let j = nums.iter().rposition(|&v| v > nums[i]).unwrap();
                nums.swap(i, j);
                nums[i + 1..].reverse();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 3, 2]);
    }

    #[test]
    fn ex2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 2, 3]);
    }

    #[test]
    fn ex3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 5, 1]);
    }

    #[test]
    fn myex1() {
        let mut nums = vec![1, 2, 3, 4];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 2, 4, 3]);
    }

    #[test]
    fn myex2() {
        let mut nums = vec![1, 2, 4, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 3, 2, 4]);
    }

    #[test]
    fn myex0() {
        let mut nums = vec![0];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[0]);
    }

    #[test]
    fn myeq() {
        let mut nums = vec![1, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 1]);
    }

    #[test]
    fn myex3() {
        let mut nums = vec![1, 1, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 2, 1]);
    }

    #[test]
    fn myex4() {
        let mut nums = vec![1, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[2, 1, 1]);
    }

    #[test]
    fn myex5() {
        let mut nums = vec![2, 1, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, &[1, 1, 2]);
    }
}
