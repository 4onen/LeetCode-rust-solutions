// https://leetcode.com/problems/single-number-iii/

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // Find the XOR of the two numbers we are looking for
        let xor = nums
            .iter()
            .copied()
            .reduce(std::ops::BitXor::bitxor)
            .unwrap();
        // Find a mask of the rightmost set bit
        let mask = xor & -xor;
        // All the numbers that have the bit in the mask are either one of
        // the two numbers we're looking for or duplicates that will cancel.
        let mut result_r = 0;
        // All the numbers without the bit in the mask are of the other number
        // or are duplicates that will cancel.
        let mut result_l = 0;
        // Determine the two numbers
        for num in nums {
            if num & mask == 0 {
                result_l ^= num;
            } else {
                result_r ^= num;
            }
        }
        vec![result_l, result_r]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: [i32; 2]) {
        assert!(nums.len() >= 2);
        assert!(nums.len() <= 30_000);
        let mut result = Solution::single_number(nums.to_vec());
        assert!(result.len() == 2);
        result.sort_unstable();
        assert!(nums.contains(&result[0]));
        assert!(nums.contains(&result[1]));
        assert_ne!(result[0], result[1]);
        assert_eq!(result, expected)
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 1, 3, 2, 5], [3, 5]);
    }

    #[test]
    fn ex2() {
        test(&[-1, 0], [-1, 0]);
    }

    #[test]
    fn ex3() {
        test(&[0, 1], [0, 1]);
    }
}
