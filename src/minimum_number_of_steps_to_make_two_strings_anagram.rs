// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/

pub struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_count = [0u16; 26];
        let mut t_count = [0u16; 26];

        for c in s.bytes() {
            s_count[(c - b'a') as usize] += 1;
        }

        for c in t.bytes() {
            t_count[(c - b'a') as usize] += 1;
        }

        let mut result = 0;
        for i in 0..26 {
            if s_count[i] > t_count[i] {
                result += s_count[i] - t_count[i];
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::min_steps("bab".to_string(), "aba".to_string()), 1);
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::min_steps("leetcode".to_string(), "practice".to_string()),
            5
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::min_steps("anagram".to_string(), "mangaar".to_string()),
            0
        );
    }
}
