// https://leetcode.com/problems/find-the-duplicate-number/

pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut slow, mut fast) = (0, 0);
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }
        let mut n = 0;
        while n != slow {
            n = nums[n as usize];
            slow = nums[slow as usize];
        }
        slow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    #[test]
    fn ex2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    #[test]
    fn ex3() {
        let nums = vec![3, 3, 3, 3, 3];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    #[test]
    fn myex1() {
        let nums = vec![1, 1];
        assert_eq!(Solution::find_duplicate(nums), 1);
    }

    #[test]
    fn myex2() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9];
        assert_eq!(Solution::find_duplicate(nums), 9);
    }

    #[test]
    fn myex3() {
        let nums = vec![1, 9, 2, 3, 4, 5, 6, 7, 8, 9, 9];
        assert_eq!(Solution::find_duplicate(nums), 9);
    }

    #[test]
    fn myex4() {
        let nums = vec![5, 4, 7, 8, 1, 2, 3, 1, 1, 9];
        assert_eq!(Solution::find_duplicate(nums), 1);
    }

    #[test]
    fn myex5() {
        let nums = vec![
            85, 42, 42, 42, 51, 17, 42, 42, 40, 99, 75, 42, 42, 12, 87, 42, 92, 30, 42, 42, 42, 42,
            39, 86, 42, 22, 49, 53, 42, 42, 42, 42, 33, 1, 21, 65, 70, 9, 88, 46, 42, 42, 81, 15,
            68, 42, 26, 67, 34, 31, 82, 42, 5, 42, 50, 66, 58, 42, 42, 57, 42, 42, 42, 16, 42, 42,
            42, 42, 20, 23, 42, 42, 79, 89, 45, 28, 42, 42, 7, 42, 13, 83, 74, 42, 42, 69, 43, 27,
            71, 10, 42, 72, 42, 42, 78, 98, 11, 25, 42, 2,
        ];
        assert_eq!(Solution::find_duplicate(nums), 42);
    }
}
