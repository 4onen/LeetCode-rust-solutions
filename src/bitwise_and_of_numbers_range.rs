// https://leetcode.com/problems/bitwise-and-of-numbers-range/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub const fn range_bitwise_and(left: i32, right: i32) -> i32 {
//         let merge = (left & right) as u32;
//         if merge == 0 {
//             return 0;
//         }
//         if left <= 0 {
//             panic!("left should be greater than 0")
//         }
//         if left > right {
//             panic!("left should be less than or equal to right")
//         }
//         let left = left as u32;
//         let right = right as u32;
//         let left_next_power_of_2 = 1 << (left.ilog2() + 1);
//         if left_next_power_of_2 <= right {
//             return 0;
//         }
//         let mut result = left;
//         let mut i = left + 1;
//         while i < right {
//             result &= i;
//             if result == 0 {
//                 break;
//             }
//             i += 1;
//         }
//         result &= right;
//         result as i32
//     }
// }

// Prefix sol'n
impl Solution {
    pub const fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        assert!(left <= right, "left should be less than or equal to right");
        if left == right {
            left
        } else {
            assert!(left >= 0, "left should be greater than or equal to 0");
            let mut shifts: u8 = 0;
            while left != right {
                left >>= 1;
                right >>= 1;
                shifts += 1;
            }
            left << shifts
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    }
}
