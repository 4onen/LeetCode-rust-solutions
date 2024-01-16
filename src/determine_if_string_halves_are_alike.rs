// https://leetcode.com/problems/determine-if-string-halves-are-alike/

pub struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let bytes = s.as_bytes();
        let halfway = bytes.len() >> 1;
        let first_half = &bytes[..halfway];
        let second_half = &bytes[halfway..];

        const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

        let first_half_vowels = first_half
            .into_iter()
            .map(|&c| c | 0x20) // lowercase
            .filter(|c| VOWELS.contains(c))
            .count();
        let second_half_vowels = second_half
            .into_iter()
            .map(|&c| c | 0x20) // lowercase
            .filter(|c| VOWELS.contains(c))
            .count();

        first_half_vowels == second_half_vowels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::halves_are_alike("book".to_string()), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::halves_are_alike("textbook".to_string()), false);
    }
}
