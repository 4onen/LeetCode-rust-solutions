// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/

pub struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut bytes = s.bytes().skip_while(|&b| b == b'a');

        loop {
            match bytes.next() {
                Some(b) if b == b'a' => break false,
                Some(b) if b == b'b' => (),
                Some(_) => unreachable!(), // non-a, non-b shouldn't exist
                None => break true,        // End of string
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::check_string("aaabbb".to_string()), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::check_string("abab".to_string()), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::check_string("bbb".to_string()), true);
    }
}
