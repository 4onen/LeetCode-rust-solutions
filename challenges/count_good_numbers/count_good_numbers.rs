// https://leetcode.com/problems/count-good-numbers/

pub struct Solution;

// Naive math (won't handle results overflowing u64)
// impl Solution {
//     pub const fn count_good_numbers(n: i64) -> i32 {
//         const MOD: u32 = 1_000_000_007;
//         assert!(n >= 1);
//         let n = n as u64;
//         let odd_indices = n / 2;
//         let even_indices = n - odd_indices;
//         let odd_combinations = 4u64.pow(odd_indices as u32);
//         let even_combinations = 5u64.pow(even_indices as u32);
//         ((even_combinations * odd_combinations) % MOD as u64) as i32
//     }
// }

// Binary exponentiation
impl Solution {
    pub const fn count_good_numbers(n: i64) -> i32 {
        const MOD: u32 = 1_000_000_007;
        assert!(n >= 1);
        let n = n as u64;
        let odd_indices = n / 2;
        let even_indices = n - odd_indices;
        const fn mypow(base: u64, mut exponent: u64) -> u64 {
            let mut result = 1;
            let mut base = base % MOD as u64;
            while exponent > 0 {
                if exponent % 2 == 1 {
                    result = (result * base) % MOD as u64;
                }
                base = (base * base) % MOD as u64;
                exponent /= 2;
            }
            result
        }
        let odd_combinations = mypow(4, odd_indices);
        let even_combinations = mypow(5, even_indices);
        ((even_combinations * odd_combinations) % MOD as u64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u64, expected: u32) {
        const MOD: u32 = 1_000_000_007;
        assert!(n >= 1);
        assert!(n <= 1_000_000_000_000_000);
        assert!(expected <= MOD - 1);
        assert_eq!(Solution::count_good_numbers(n as i64), expected as i32);
    }

    #[test]
    fn ex1() {
        test(1, 5)
    }

    #[test]
    fn myex2() {
        test(2, 5 * 4)
    }

    #[test]
    fn myex3() {
        test(3, 5 * 4 * 5)
    }

    #[test]
    fn ex4() {
        test(4, 400)
    }

    #[test]
    fn myex5() {
        test(5, 5 * 4 * 5 * 4 * 5)
    }

    #[test]
    fn myex6() {
        test(6, 5 * 4 * 5 * 4 * 5 * 4)
    }

    #[test]
    fn myex7() {
        test(7, 5 * 4 * 5 * 4 * 5 * 4 * 5)
    }

    #[test]
    fn myex8() {
        test(8, 5 * 4 * 5 * 4 * 5 * 4 * 5 * 4)
    }

    #[test]
    fn myex9() {
        test(9, 5 * 4 * 5 * 4 * 5 * 4 * 5 * 4 * 5)
    }

    #[test]
    fn ex50() {
        test(50, 564908303)
    }

    #[test]
    fn my_extreme_ex1() {
        test(1_000_000_000_000_000, 711414395)
    }
}
