// https://leetcode.com/problems/check-if-bitwise-or-has-trailing-zeros/

pub struct Solution;

// Loop sol'n
// impl Solution {
//     pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
//         let mut found_first_element = false;
//         for n in nums {
//             if n & 1 == 0 {
//                 if found_first_element {
//                     return true;
//                 } else {
//                     found_first_element = true;
//                 }
//             }
//         }
//         false
//     }
// }

// Iterator sol'n
impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut iter = nums.iter();
        iter.any(|&n| n & 1 == 0) && iter.any(|&n| n & 1 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::has_trailing_zeros(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::has_trailing_zeros(vec![2, 4, 8, 16]), true);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::has_trailing_zeros(vec![1, 3, 5, 7, 9]), false);
    }
}
