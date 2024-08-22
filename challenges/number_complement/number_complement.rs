// https://leetcode.com/problems/number-complement/

pub struct Solution;

// AND Mask sol'n
impl Solution {
    pub const fn find_complement(num: i32) -> i32 {
        (!num) & (i32::MAX >> num.leading_zeros())
    }
}

// XOR Mask sol'n
// impl Solution {
//     pub const fn find_complement(num: i32) -> i32 {
//         num ^ (i32::MAX >> (num.leading_zeros()-1))
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    fn test(num: u32, expected: u32) {
        assert!(num >= 1);
        assert!(num <= i32::MAX as u32);
        assert!(expected <= i32::MAX as u32);
        assert_eq!(Solution::find_complement(num as i32), expected as i32);
    }

    #[test]
    fn ex1() {
        test(5,2)
    }

    #[test]
    fn ex2() {
        test(1,0)
    }

    #[test]
    fn discussion_case1() {
        test(i32::MAX as u32, 0)
    }
}
