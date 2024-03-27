// https://leetcode.com/problems/subarray-product-less-than-k/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
//         match k {
//             i32::MIN..=1 => 0,
//             2..=1_000_000 => {
//                 let mut count = 0i32;
//                 let mut product = 1;
//                 let mut left = 0;
//                 let mut right = 0;
//                 while right < nums.len() {
//                     product *= nums[right];
//                     while product >= k {
//                         product /= nums[left];
//                         left += 1;
//                     }
//                     right += 1;
//                     count += (right - left) as i32;
//                 }
//                 count
//             }
//             _ => unreachable!("k is out of range"),
//         }
//     }
// }

// Improve loop to avoid bounds checks
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            0
        } else {
            let mut count = 0;
            let mut product = 1;
            let mut left = 0;
            for right in 0..nums.len() {
                product *= nums[right];
                while product >= k {
                    product /= nums[left];
                    left += 1;
                }
                count += right - left + 1;
            }
            count as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 8);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 2, 3];
        let k = 0;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 0);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 1;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 0);
    }

    #[test]
    fn myex1() {
        let nums = vec![1];
        let k = 2;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 1);
    }

    #[test]
    fn myex11() {
        let nums = vec![1, 1];
        let k = 2;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 3);
    }

    #[test]
    fn myex111() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 6);
    }

    #[test]
    fn discussion_case80() {
        let nums = vec![1; 8212];
        let k = 1;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 0);
    }

    #[test]
    fn discussion_case2() {
        let nums = vec![2, 5, 10, 6];
        let k = 30;
        assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), 5);
    }
}
