// https://leetcode.com/problems/majority-element/

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut most_common = 0;
        let mut count: usize = 0;
        for num in nums {
            if count == 0 {
                most_common = num;
            }
            if most_common == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        return most_common;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::majority_element(vec![42]), 42);
    }
}
