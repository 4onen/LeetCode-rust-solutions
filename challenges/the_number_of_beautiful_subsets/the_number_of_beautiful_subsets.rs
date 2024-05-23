// https://leetcode.com/problems/the-number-of-beautiful-subsets/

pub struct Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        fn dfs(nums: &[i32], k: i32, index: u8, path: &mut Vec<i32>) -> i32 {
            if index == nums.len() as u8 {
                return (path.len() > 0) as i32;
            }
            // Try without current number
            let mut result = dfs(nums, k, index + 1, path);
            // Check if current number can be added to the path
            // Remember we can add a number to the set only if
            // no other number in the set has an absolute difference
            // with the current number equal to k
            let can_add = {
                let current = nums[index as usize];
                path.iter().all(|&num| (current - num).abs() != k)
            };
            if can_add {
                path.push(nums[index as usize]);
                result += dfs(nums, k, index + 1, path);
                path.pop();
            }
            result
        }
        dfs(&nums, k, 0, &mut Vec::with_capacity(nums.len() as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 20);
        assert!(k >= 1);
        assert!(k <= 1000);
        for num in nums {
            assert!(*num >= 1);
            assert!(*num <= 1000);
        }
        let result = Solution::beautiful_subsets(nums.to_vec(), k);
        assert!(result >= nums.len() as i32);
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 4, 6], 2, 4)
    }

    #[test]
    fn ex2() {
        test(&[1], 1, 1)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 3], 3, 3);
    }

    #[test]
    fn discussion_case2() {
        test(&[1000, 1], 999, 2);
    }

    #[test]
    fn discussion_case3() {
        test(
            &[2, 4, 6, 8, 10, 12, 14, 15, 16, 17, 18, 100, 103, 106],
            3,
            4799,
        );
    }

    #[test]
    fn discussion_case4() {
        test(&[1000, 999, 998, 997, 996, 995, 994, 993, 992, 991], 1, 143)
    }

    #[test]
    fn discussion_case5() {
        test(&[10, 4, 5, 7, 2, 1], 3, 23)
    }
}
