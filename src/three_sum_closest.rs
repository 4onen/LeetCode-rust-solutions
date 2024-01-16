// https://leetcode.com/problems/3sum-closest/

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                match sum.cmp(&target) {
                    std::cmp::Ordering::Greater => k -= 1,
                    std::cmp::Ordering::Less => j += 1,
                    std::cmp::Ordering::Equal => return target,
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], 100), 3);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], 0), 2);
    }
}
