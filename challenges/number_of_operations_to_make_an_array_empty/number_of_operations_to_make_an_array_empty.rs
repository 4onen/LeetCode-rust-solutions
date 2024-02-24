// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/

pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut operations = 0;

        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() && nums[i] == nums[j] {
                j += 1;
            }

            let count = (j - i) as u32;
            match count % 3 {
                0 => {
                    operations += count / 3;
                }
                1 => {
                    if count > 3 {
                        operations += (count - 4) / 3 + 2;
                    } else {
                        return -1;
                    }
                }
                2 => {
                    operations += (count - 2) / 3 + 1;
                }
                _ => unreachable!(),
            }

            i = j;
        }

        operations as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::min_operations(vec![2, 1, 2, 2, 3, 3]), -1);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::min_operations(vec![1, 1]), 1);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::min_operations(vec![1, 2]), -1);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3]), -1);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::min_operations(vec![1, 1, 2]), -1);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 2]), 2);
    }
}
