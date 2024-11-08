// https://leetcode.com/problems/maximum-xor-for-each-query/

pub struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut prefix_xor = 0;
        for num in nums.iter() {
            prefix_xor ^= num;
        }
        let mut result = std::vec::Vec::with_capacity(nums.len());
        let maximum_bit_mask = (1 << maximum_bit) - 1;
        for num in nums.into_iter().rev() {
            result.push((!prefix_xor) & maximum_bit_mask);
            prefix_xor ^= num;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], maximum_bit: i32, expected: &[i32]) {
        assert!(nums.len() == expected.len());
        assert_eq!(
            Solution::get_maximum_xor(nums.to_vec(), maximum_bit),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 1, 3], 2, &[0, 3, 2, 3])
    }

    #[test]
    fn ex2() {
        test(&[2, 3, 4, 7], 3, &[5, 2, 6, 5])
    }

    #[test]
    fn ex3() {
        test(&[0, 1, 2, 2, 5, 7], 3, &[4, 3, 6, 4, 6, 7])
    }
}
