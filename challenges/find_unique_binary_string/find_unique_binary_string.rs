// https://leetcode.com/problems/find-unique-binary-string/

pub struct Solution;

// Naive try-everything sol'n (Forgot to test 16-bit case)
// impl Solution {
//     pub fn find_different_binary_string(nums: Vec<String>) -> String {
//         let n = nums.len() as u8;
//         let nums = nums
//             .into_iter()
//             .map(|s| {
//                 let mut acc: u16 = 0;
//                 for b in s.into_bytes() {
//                     acc <<= 1;
//                     acc |= (b & 1) as u16;
//                 }
//                 acc
//             })
//             .collect::<std::collections::HashSet<_>>();
//         for i in 0..(1 << (nums.len() + 1)) {
//             if !nums.contains(&i) {
//                 let mut result = vec![0u8; n as usize];
//                 let mut i = i;
//                 for b in (0..n).rev() {
//                     result[b as usize] = b'0' + (i & 1) as u8;
//                     i >>= 1;
//                 }
//                 return unsafe { std::string::String::from_utf8_unchecked(result) };
//             }
//         }
//         unreachable!()
//     }
// }

// Naive try-everything sol'n (corrected iter range for larger n)
// impl Solution {
//     pub fn find_different_binary_string(nums: Vec<String>) -> String {
//         let n = nums.len() as u8;
//         let nums = nums
//             .into_iter()
//             .map(|s| {
//                 let mut acc: u16 = 0;
//                 for b in s.into_bytes() {
//                     acc <<= 1;
//                     acc |= (b & 1) as u16;
//                 }
//                 acc
//             })
//             .collect::<std::collections::HashSet<_>>();
//         let range_end = if n < 16 { (1u16 << n) - 1 } else { u16::MAX };
//         for i in 0..=range_end {
//             if !nums.contains(&i) {
//                 let mut result = vec![0u8; n as usize];
//                 let mut i = i;
//                 for b in (0..n).rev() {
//                     result[b as usize] = b'0' + (i & 1) as u8;
//                     i >>= 1;
//                 }
//                 return unsafe { std::string::String::from_utf8_unchecked(result) };
//             }
//         }
//         unreachable!()
//     }
// }

// Cantor's diagonalization sol'n
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut result = vec![0u8; n];
        for idx in 0..n {
            result[idx] = nums[idx].as_bytes()[idx] ^ 1;
        }
        unsafe { std::string::String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[&str]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 16);
        let mut seen = std::collections::HashSet::new();
        for &num in nums {
            assert_eq!(num.len(), nums.len());
            for &b in num.as_bytes() {
                assert!(b == b'0' || b == b'1');
            }
            assert!(seen.insert(num));
        }
        let result =
            Solution::find_different_binary_string(nums.iter().map(|&x| x.to_string()).collect());
        for &num in nums {
            assert_ne!(result, num);
        }
    }

    #[test]
    fn ex1() {
        test(&["01", "10"])
    }

    #[test]
    fn ex2() {
        test(&["00", "01"])
    }

    #[test]
    fn ex3() {
        test(&["111", "011", "001"])
    }

    #[test]
    fn failing_case1() {
        test(&[
            "0000000000001100",
            "0000000000001101",
            "0000000000000111",
            "0000000000000110",
            "0000000000000011",
            "1111111111111111",
            "0000000000000101",
            "0000000000001000",
            "0000000000001111",
            "0000000000001110",
            "0000000000000001",
            "0000000000000000",
            "0000000000001001",
            "0000000000000100",
            "0000000000001011",
            "0000000000001010",
        ])
    }
}
