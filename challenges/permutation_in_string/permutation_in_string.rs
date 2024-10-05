// https://leetcode.com/problems/permutation-in-string/

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut s1_count = [0u16; 26];
        let mut s2_count = [0u16; 26];
        if s1.len() > s2.len() {
            return false;
        }
        for i in 0..s1.len() {
            s1_count[(s1[i] - b'a') as usize] += 1;
            s2_count[(s2[i] - b'a') as usize] += 1;
        }
        if s1_count == s2_count {
            return true;
        }
        for i in s1.len()..s2.len() {
            s2_count[(s2[i] - b'a') as usize] += 1;
            s2_count[(s2[i - s1.len()] - b'a') as usize] -= 1;
            if s1_count == s2_count {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s1: &str, s2: &str, expected: bool) {
        assert_eq!(
            Solution::check_inclusion(s1.to_string(), s2.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("ab", "eidbaooo", true)
    }

    #[test]
    fn ex2() {
        test("ab", "eidboaoo", false)
    }
}
