// https://leetcode.com/problems/longest-common-prefix/

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].as_bytes();
        for s in strs[1..].into_iter() {
            let bytesiter = s.as_bytes().into_iter();
            let matching_count = std::iter::zip(prefix, bytesiter)
                .take_while(|(a, b)| a == b)
                .count();
            prefix = &prefix[..matching_count];
        }
        unsafe { std::str::from_utf8_unchecked(prefix) }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn ex2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
    }
}
