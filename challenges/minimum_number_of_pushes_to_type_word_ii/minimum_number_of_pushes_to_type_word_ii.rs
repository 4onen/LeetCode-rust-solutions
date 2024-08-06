// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn minimum_pushes(word: String) -> i32 {
//         let mut counts = [0i32; 26];
//         for b in word.bytes() {
//             counts[(b - b'a') as usize] += 1;
//         }
//         counts.sort_unstable();
//         let mut result = 0;
//         for i in (0..26u8).rev() {
//             if counts[i as usize] == 0 {
//                 break;
//             }
//             result += (counts[i as usize]) * (1 + (25 - i as i32) / 8);
//         }
//         result as i32
//     }
// }

// Eliminate some branching
// impl Solution {
//     pub fn minimum_pushes(word: String) -> i32 {
//         let mut counts = [0i32; 26];
//         for b in word.bytes() {
//             counts[(b - b'a') as usize] += 1;
//         }
//         counts.sort_unstable();
//         let mut result = 0;
//         for i in (0..26u8).rev() {
//             result += (counts[i as usize]) * (1 + (25 - i as i32) / 8);
//         }
//         result as i32
//     }
// }

// Try to use iterators more heavily to improve opt
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = [0i32; 26];
        for b in word.into_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        counts.sort_unstable();
        let mut result = 0;
        for (i, c) in counts.into_iter().rev().enumerate() {
            result += c * (1 + (i as i32) / 8);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(word: &str, expected: i32) {
        assert!(word.len() >= 1);
        assert!(word.len() <= 100_000);
        for b in word.bytes() {
            assert!(b.is_ascii_lowercase());
        }
        assert_eq!(Solution::minimum_pushes(word.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("abcde", 5);
    }

    #[test]
    fn ex2() {
        test("xyzxyzxyzxyz", 12);
    }

    #[test]
    fn ex3() {
        test("aabbccddeeffgghhiiiiii", 24);
    }

    #[test]
    fn failing_case1() {
        test("alporfmdqsbhncwyu", 27);
    }
}
