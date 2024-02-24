// https://leetcode.com/problems/search-in-rotated-sorted-array/

pub struct Solution;

// Actual binary search
// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//         let mut left = 0;
//         let mut right = nums.len() - 1;
//         loop {
//             let mid = (left + right) / 2;
//             if nums[mid] == target {
//                 break mid as i32;
//             }
//             if nums[left] <= nums[mid] {
//                 if nums[left] <= target && target < nums[mid] {
//                     right = mid - 1;
//                 } else {
//                     left = mid + 1
//                 }
//             } else {
//                 if nums[mid] < target && target <= nums[right] {
//                     left = mid + 1
//                 } else {
//                     right = mid - 1
//                 }
//             }
//             if left > right {
//                 break -1;
//             }
//         }
//     }
// }

// Cheesy linear search
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.into_iter()
            .position(|x| x == target)
            .map(|x| x as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::search(vec![1], 1), 0);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::search(vec![1], 2), -1);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::search(vec![1, 3], 1), 0);
        assert_eq!(Solution::search(vec![3, 1], 1), 1);
    }
}
