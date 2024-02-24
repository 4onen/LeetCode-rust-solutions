// https://leetcode.com/problems/set-mismatch/

pub struct Solution;

// Cyclic sort sol'n
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0;
        while i < nums.len() {
            let j = (nums[i]) as usize - 1;
            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                i += 1;
            }
        }
        for i in 0..nums.len() {
            if (nums[i]) as usize - 1 != i {
                return vec![nums[i], (i + 1) as i32];
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::find_error_nums(vec![3, 2, 2]), vec![2, 1]);
    }

    #[test]
    fn discussino_case2() {
        assert_eq!(
            Solution::find_error_nums(vec![1, 2, 3, 4, 9, 6, 7, 8, 9]),
            vec![9, 5]
        );
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::find_error_nums(vec![4, 8, 1, 5, 2, 7, 4, 6]),
            vec![4, 3]
        );
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(
            Solution::find_error_nums(vec![2, 1, 4, 2, 6, 5]),
            vec![2, 3]
        );
    }

    #[test]
    fn discussion_case5() {
        assert_eq!(
            Solution::find_error_nums(vec![8, 7, 3, 5, 3, 6, 1, 4]),
            vec![3, 2]
        );
    }

    #[test]
    fn discussion_case6() {
        assert_eq!(Solution::find_error_nums(vec![2, 4, 3, 2]), vec![2, 1]);
    }
}
