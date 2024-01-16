// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

pub struct Solution;

// partition_point solution
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left_point = nums.partition_point(|&x| x < target);
        let right_point = nums.partition_point(|&x| x <= target);

        if left_point == right_point {
            vec![-1, -1]
        } else {
            vec![left_point as i32, right_point as i32 - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
