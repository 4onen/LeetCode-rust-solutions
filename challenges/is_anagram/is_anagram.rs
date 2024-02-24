// https://leetcode.com/problems/valid-anagram/

pub struct Solution;

// Solution 1: Sort
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_bytes = s.into_bytes();
        let mut t_bytes = t.into_bytes();
        s_bytes.sort();
        t_bytes.sort();
        s_bytes == t_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }
}
