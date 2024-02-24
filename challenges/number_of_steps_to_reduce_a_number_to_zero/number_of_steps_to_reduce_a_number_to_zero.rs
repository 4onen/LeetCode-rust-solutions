// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/

pub struct Solution;

// impl Solution {
//     pub fn number_of_steps(num: i32) -> i32 {
//         (num.count_ones() + (i32::BITS - (1 + num.leading_zeros()).min(32))) as i32
//     }
// }

// Const-fn sol'n
impl Solution {
    pub const fn number_of_steps(num: i32) -> i32 {
        let leading_zeros_plus_one = 1 + num.leading_zeros();
        let capped = if leading_zeros_plus_one > 32 {
            32
        } else {
            leading_zeros_plus_one
        };
        (num.count_ones() + (i32::BITS - capped)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }

    #[test]
    fn failing_case() {
        assert_eq!(Solution::number_of_steps(0), 0);
    }
}
