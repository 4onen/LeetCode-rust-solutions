// https://leetcode.com/problems/product-of-array-except-self/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
//         let prefixes: Vec<i32> = nums
//             .iter()
//             .scan(1, |acc, &x| {
//                 *acc *= x;
//                 Some(*acc)
//             })
//             .collect();
//         let mut suffixes: Vec<i32> = nums
//             .iter()
//             .skip(1)
//             .rev()
//             .scan(1, |acc, &x| {
//                 *acc *= x;
//                 Some(*acc)
//             })
//             .collect();
//         suffixes.reverse();
//         let n = nums.len();
//         nums[0] = suffixes[0];
//         nums[n - 1] = prefixes[prefixes.len() - 2];
//         for i in 1..nums.len() - 1 {
//             nums[i] = prefixes[i - 1] * suffixes[i];
//         }
//         nums
//     }
// }

// O(1) "extra space" sol'n v1
impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums.clone();
        for i in 1..nums.len() {
            nums[i] *= nums[i - 1];
        }
        res[0] = 1;
        for i in (0..res.len() - 1).rev() {
            res[i] *= res[i + 1];
        }
        for i in 1..res.len() - 1 {
            let left = nums[i - 1];
            let right = res[i + 1];
            res[i] = left * right;
        }
        res[nums.len() - 1] = nums[nums.len() - 2];
        res
    }
}

// O(1) "extra space" sol'n merge two passes
// impl Solution {
//     pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//         let mut res = vec![0; nums.len()];
//         res[0] = 1;
//         res[1..].copy_from_slice(&nums[..nums.len() - 1]);
//         for i in 1..res.len() {
//             res[i] *= res[i - 1];
//         }
//         let mut right = 1;
//         for i in (0..res.len()).rev() {
//             res[i] *= right;
//             right *= nums[i];
//         }
//         res
//     }
// }

// O(1) "extra space" sol'n merge three more passes (two inits go away!)
// impl Solution {
//     pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//         let n = nums.len();
//         let mut res = std::vec::Vec::with_capacity(n);
//         let uninit = res.spare_capacity_mut();
//         uninit[0].write(1);
//         for i in 1..n {
//             uninit[i].write(nums[i - 1] * unsafe { uninit[i - 1].assume_init() });
//         }
//         unsafe { res.set_len(n) };
//         let mut right = 1;
//         for i in (0..res.len()).rev() {
//             res[i] *= right;
//             right *= nums[i];
//         }
//         res
//     }
// }
// ... and somehow it's still slower than my first O(1) "extra space" sol'n v1

// O(1) "extra space" sol'n merges flipped
// impl Solution {
//     pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//         let n = nums.len();
//         let mut res = std::vec::Vec::with_capacity(nums.len());
//         let uninit = &mut res.spare_capacity_mut()[..n];
//         uninit[n - 1].write(nums[n - 1]);
//         for i in (0..n - 1).rev() {
//             uninit[i].write(nums[i] * unsafe { uninit[i + 1].assume_init() });
//         }
//         unsafe { res.set_len(nums.len()) };
//         let mut left = 1;
//         for i in 0..n - 1 {
//             let right = res[i + 1];
//             res[i] = left * right;
//             left *= nums[i];
//         }
//         res[n - 1] = left;
//         res
//     }
// }
// And this is my slowest "O(1) extra space" sol'n yet!
// Why do I feel like spare_capacity_mut bound checks are slowing me down?
// AHA! It was the bounds checks! Added [..n] and it sped way up!
// But still slower than my first O(1) "extra space" sol'n
// Sigh. Can't beat word-aligned memcpy.

// O(1) "extra space" sol'n with two merged passes v2
// impl Solution {
//     pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//         let mut res = nums.clone();
//         res[0] = 1;
//         for i in (0..res.len() - 1).rev() {
//             res[i] *= res[i + 1];
//         }
//         let mut left = 1;
//         for i in 0..res.len() - 1 {
//             res[i] = left * res[i + 1];
//             left *= nums[i];
//         }
//         res[nums.len() - 1] = left;
//         res
//     }
// }
// Ugh. Nope. Still slower than my first O(1) "extra space" sol'n

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn partial_discussion_case1() {
        let nums = [
            -1, -1, 1, -1, -1, 1, -1, -1, -1, 1, 1, -1, 1, 1, 1, 1, -1, 1, 1, -1, -1, 1, -1, -1,
            -1, 1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, -1,
        ];
        let expected = nums;
        assert_eq!(Solution::product_except_self(nums.to_vec()), expected);
    }

    #[test]
    fn partial_discussion_case2() {
        let nums = [
            -1, -1, 1, -1, -1, 1, -1, -1, -1, 1, 1, -1, 1, 1, 1, 1, -1, 1, 1, -1, -1, 1, -1, -1,
            -1, 1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, -1, -1,
        ];
        let expected = nums.map(i32::wrapping_neg);
        assert_eq!(Solution::product_except_self(nums.to_vec()), expected);
    }
}
