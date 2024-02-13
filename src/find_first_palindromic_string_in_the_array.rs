// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/

pub struct Solution;

// Braindead sol'n
// impl Solution {
//     pub fn first_palindrome(words: Vec<String>) -> String {
//         const fn is_palindrome(s: &[u8]) -> bool {
//             let mut i = 0;
//             let mut j = s.len() - 1;
//             while i < j {
//                 if s[i] != s[j] {
//                     return false;
//                 }
//                 i += 1;
//                 j -= 1;
//             }
//             true
//         }
//         words
//             .into_iter()
//             .find(|s| is_palindrome(s.as_bytes()))
//             .unwrap_or_else(String::new)
//     }
// }

// In-lambda sol'n
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|s| {
                std::iter::zip(s.bytes(), s.bytes().rev())
                    .take(s.len() / 2)
                    .all(|(a, b)| a == b)
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string_vec<'a>(arr: &'a [&'a str]) -> Vec<String> {
        arr.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn ex1() {
        let input = ["abc", "car", "ada", "racecar", "cool"];
        assert_eq!(Solution::first_palindrome(to_string_vec(&input)), "ada");
    }

    #[test]
    fn ex2() {
        let input = ["notapalindrome", "racecar"];
        assert_eq!(Solution::first_palindrome(to_string_vec(&input)), "racecar");
    }

    #[test]
    fn ex3() {
        let input = ["def", "ghi"];
        assert_eq!(Solution::first_palindrome(to_string_vec(&input)), "");
    }

    #[test]
    fn myex1() {
        let input = ["def", "ghi", "racecar"];
        assert_eq!(Solution::first_palindrome(to_string_vec(&input)), "racecar");
    }

    #[test]
    fn myex2() {
        let input = ["def", "ghi", "racecar", "aceca"];
        assert_eq!(Solution::first_palindrome(to_string_vec(&input)), "racecar");
    }
}
