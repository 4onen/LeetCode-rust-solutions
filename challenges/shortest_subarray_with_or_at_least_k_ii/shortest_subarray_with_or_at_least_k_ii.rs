// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/

pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        const N_BITS: usize = 30;
        let mut bits = [0u32; N_BITS];
        let mut or = 0;
        let mut i = 0u32;
        let mut j = 0u32;
        let mut min = u32::MAX;
        while j < nums.len() as u32 {
            let num = nums[j as usize];
            or |= num;
            for b in 0..N_BITS {
                bits[b] += if num & (1 << b) != 0 { 1 } else { 0 };
            }
            while or >= k {
                min = std::cmp::min(min, j + 1 - i);
                if min <= 1 {
                    return min as i32;
                }
                let num = nums[i as usize];
                for b in 0..N_BITS {
                    if num & (1 << b) != 0 {
                        bits[b] -= 1;
                        if bits[b] <= 0 {
                            or &= !(1 << b);
                        }
                    }
                }
                i += 1;
            }
            j += 1;
        }
        // Safety: min is u32::MAX if no solution is found, which by two's
        // complement representation is the same as the -1i32.
        unsafe { std::mem::transmute(min) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 200_000);
        assert!(k >= 0);
        assert!(k <= 1_000_000_000);
        assert!(expected >= 1 || expected == -1);
        assert!(expected <= nums.len() as i32);
        for &num in nums {
            assert!(num >= 0);
            assert!(num <= 1_000_000_000);
        }
        assert_eq!(
            Solution::minimum_subarray_length(nums.to_vec(), k),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3], 2, 1)
    }

    #[test]
    fn ex2() {
        test(&[2, 1, 8], 10, 3)
    }

    #[test]
    fn ex3() {
        test(&[1, 2], 0, 1)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 2, 32, 21], 55, 3)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 71, 26, 12, 2, 60, 2], 96, 5)
    }

    #[test]
    fn discussion_case3() {
        test(&[5, 1, 32, 86, 2], 117, 2)
    }

    #[test]
    fn discussion_case4() {
        test(&[2, 1, 6, 32, 86, 56], 123, 2)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[1; 200_000], 2, -1)
    }

    #[test]
    fn my_extreme_ex2() {
        test(&[1; 200_000], 1, 1)
    }

    #[test]
    fn my_extreme_ex3() {
        test(&[1; 200_000], 0, 1)
    }

    #[test]
    fn my_extreme_ex4() {
        let mut input = [1; 200_000];
        input[199_999] = 2;
        test(&input, 2, 1)
    }

    #[test]
    fn my_extreme_ex5() {
        let mut input = [1; 200_000];
        input[199_999] = 2;
        test(&input, 1, 1)
    }

    #[test]
    fn my_extreme_ex6() {
        let mut input = [1; 200_000];
        input[199_999] = 2;
        test(&input, 3, 2)
    }
}
