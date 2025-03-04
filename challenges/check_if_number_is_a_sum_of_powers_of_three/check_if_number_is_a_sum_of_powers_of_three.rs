// https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/

pub struct Solution;

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        const N_POWERS: u32 = 15; // 3^15 = 14_348_907 > 10_000_000... but apparently we need it anyway?
        const POWERS: [u32; N_POWERS as usize] = {
            let mut result = [0u32; N_POWERS as usize];
            let mut i = 0;
            while i < N_POWERS {
                result[i as usize] = 3u32.pow(i);
                i += 1;
            }
            result
        };
        let mut n = n as u32;
        for &power in POWERS.iter().rev() {
            if n >= power {
                n -= power;
            }
            if n >= power {
                return false;
            }
        }
        n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u32, expected: bool) {
        assert!(n >= 1);
        assert!(n <= 10_000_000);
        assert_eq!(Solution::check_powers_of_three(n as i32), expected);
    }

    #[test]
    fn ex1() {
        test(12, true)
    }

    #[test]
    fn ex2() {
        test(91, true)
    }

    #[test]
    fn ex3() {
        test(21, false)
    }

    #[test]
    fn my_extreme_ex1() {
        test(10_000_000, false)
    }

    #[test]
    fn failing_case1() {
        test(6_378_022, true)
    }

    #[test]
    fn my_extreme_ex2() {
        test(9_999_999, false)
    }
}
