// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/

pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        const N_BITS: usize = 24;
        let mut bit_appearances = [0; N_BITS];
        for &num in &candidates {
            for i in 0..N_BITS {
                if num & (1 << i) != 0 {
                    bit_appearances[i] += 1;
                }
            }
        }
        bit_appearances.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(candidates: &[i32], expected: i32) {
        assert_eq!(Solution::largest_combination(candidates.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[16, 17, 71, 62, 12, 24, 14], 4)
    }

    #[test]
    fn ex2() {
        test(&[8, 8], 2)
    }

    #[test]
    fn myex1() {
        test(&[1, 2, 2, 3, 3, 4, 4, 6, 6], 6)
    }

    #[test]
    fn myex2() {
        test(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 5)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[10_000_000; 100_000], 100_000)
    }

    #[test]
    fn my_extreme_ex2() {
        test(&[10_000_000, 9_999_999, 9_999_998, 9_999_997, 9_999_996], 5)
    }
}
