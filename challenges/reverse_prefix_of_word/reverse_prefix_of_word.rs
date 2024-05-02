// https://leetcode.com/problems/reverse-prefix-of-word/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn reverse_prefix(word: String, ch: char) -> String {
//         assert!(('a'..='z').contains(&ch));
//         let ch = ch as u8;
//         let mut word = word.into_bytes();
//         assert!(word.len() <= 250);
//         let mut first_occurrence = 0u8;
//         while first_occurrence < word.len() as u8 {
//             if word[first_occurrence as usize] == ch {
//                 break;
//             }
//             first_occurrence += 1;
//         }
//         if first_occurrence < word.len() as u8 {
//             word[..=first_occurrence as usize].reverse();
//         }
//         unsafe { String::from_utf8_unchecked(word) }
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(first_occurrence) = word.find(ch) {
            let mut word = word.into_bytes();
            word[..=first_occurrence].reverse();
            unsafe { String::from_utf8_unchecked(word) }
        } else {
            word
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(word: &str, ch: u8, expected: &str) {
        assert_eq!(
            Solution::reverse_prefix(word.to_string(), ch as char),
            expected.to_string()
        );
    }

    #[test]
    fn ex1() {
        test("abcdefd", 'd' as u8, "dcbaefd");
    }

    #[test]
    fn ex2() {
        test("xyxzxe", 'z' as u8, "zxyxxe");
    }

    #[test]
    fn ex3() {
        test("abcd", 'z' as u8, "abcd");
    }
}
