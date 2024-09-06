// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii/

pub struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut even = [0u16; 26];
        let mut odd = [0u16; 26];
        for (i,b) in s1.bytes().enumerate() {
            if i % 2 == 0 {
                even[(b - b'a') as usize] = even[(b - b'a') as usize].wrapping_add(1);
            } else {
                odd[(b - b'a') as usize] = odd[(b - b'a') as usize].wrapping_add(1);
            }
        }
        for (i,b) in s2.bytes().enumerate() {
            if i % 2 == 0 {
                even[(b - b'a') as usize] = even[(b - b'a') as usize].wrapping_sub(1);
            } else {
                odd[(b - b'a') as usize] = odd[(b - b'a') as usize].wrapping_sub(1);
            }
        }
        for i in 0..26 {
            if even[i] != 0 || odd[i] != 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s1: &str, s2: &str, expected: bool) {
        assert_eq!(Solution::check_strings(s1.to_string(), s2.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test(
            "abcdba", "cabdab",
            true
        )
    }

    #[test]
    fn ex2() {
        test(
            "abe", "bea",
            false
        )
    }
}
