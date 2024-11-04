// https://leetcode.com/problems/string-compression-iii/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn compressed_string(word: String) -> String {
        let word = word.as_bytes();
        let mut result = std::vec::Vec::new();
        let mut i = 0;
        while i < word.len() {
            let mut j = i + 1;
            while j < word.len() && j - i < 9 && word[j] == word[i] {
                j += 1;
            }
            let count = (j - i) as u8;
            result.push(count + b'0');
            result.push(word[i]);
            i = j;
        }
        return unsafe { String::from_utf8_unchecked(result) };
    }
}

// Reduced allocations (slower, no change to LeetCode measured mem usage)
// impl Solution {
//     pub fn compressed_string(word: String) -> String {
//         let word = word.as_bytes();
//         let mut result = std::vec::Vec::with_capacity(word.len() * 2); // Worst-case scenario
//         let mut i = 0;
//         while i < word.len() {
//             let mut j = i + 1;
//             while j < word.len() && j - i < 9 && word[j] == word[i] {
//                 j += 1;
//             }
//             let count = (j - i) as u8;
//             result.extend_from_slice(&[count + b'0', word[i]]);
//             i = j;
//         }
//         return unsafe { String::from_utf8_unchecked(result) };
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(word: &str, expected: &str) {
        assert!(word.len() >= 1);
        assert!(word.len() <= 200_000);
        assert_eq!(Solution::compressed_string(word.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("abcde", "1a1b1c1d1e")
    }

    #[test]
    fn ex2() {
        test("aaaaaaaaaaaaaabb", "9a5a2b")
    }
}
