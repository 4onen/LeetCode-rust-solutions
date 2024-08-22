// https://leetcode.com/problems/complement-of-base-10-integer/

pub struct Solution;

// AND Mask sol'n
impl Solution {
    pub const fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {return 1;}
        (!n) & (i32::MAX >> n.leading_zeros())
    }
}

// XOR Mask sol'n
// impl Solution {
//     pub const fn bitwise_complement(n: i32) -> i32 {
//         if n == 0 {return 1;}
//         n ^ (i32::MAX >> (n.leading_zeros() - 1))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u32, expected: u32) {
        assert!(n <= 1_000_000_000);
        assert_eq!(Solution::bitwise_complement(n as i32), expected as i32);
    }

    #[test]
    fn ex1() {
        test(5, 2);
    }

    #[test]
    fn ex2() {
        test(7, 0);
    }

    #[test]
    fn ex3() {
        test(10, 5);
    }

    #[test]
    fn discussion_case1() {
        test(0, 1); // Weird logic but w/e
    }
}
