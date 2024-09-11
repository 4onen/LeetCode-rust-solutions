// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/

pub struct Solution;

impl Solution {
    pub const fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(start: i32, goal: i32, expected: i32) {
        assert_eq!(Solution::min_bit_flips(start, goal), expected);
    }

    #[test]
    fn ex1() {
        test(
            10,
            7,
            3,
        )
    }

    #[test]
    fn ex2() {
        test(
            3,
            4,
            3,
        )
    }
}
