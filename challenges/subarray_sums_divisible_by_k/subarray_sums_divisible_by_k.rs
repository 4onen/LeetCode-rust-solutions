// https://leetcode.com/problems/subarray-sums-divisible-by-k/

pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut subarrays = 0;
        let mut mod_space = vec![0; k as usize];
        mod_space[0] = 1;
        for num in nums {
            sum += num;
            let mod_val = sum.rem_euclid(k);
            subarrays += mod_space[mod_val as usize];
            mod_space[mod_val as usize] += 1;
        }
        subarrays
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i16], k: u16, expected: i32) {
        assert!(k >= 2);
        assert!(k <= 10_000);
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 30_000);
        for num in nums {
            assert!(*num >= -10_000);
            assert!(*num <= 10_000);
        }
        let input = nums.iter().map(|&x| x as i32).collect();
        assert_eq!(Solution::subarrays_div_by_k(input, k as i32), expected);
    }

    #[test]
    fn ex1() {
        test(&[4, 5, 0, -2, -3, 1], 5, 7);
    }

    #[test]
    fn ex2() {
        test(&[5], 9, 0);
    }
}
