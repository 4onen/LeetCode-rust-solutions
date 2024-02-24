// https://leetcode.com/problems/longest-substring-without-repeating-characters/

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let bytes = s.as_bytes();
        let mut char_last_index = [u16::MAX; 128 - 32 - 1];
        let mut max_len: u16 = std::cmp::min(1, bytes.len()) as u16;
        let mut start = 0;
        char_last_index[(bytes[0] - b' ') as usize] = 0;
        for i in 1..bytes.len() {
            let c = (bytes[i] - b' ') as usize;
            start = std::cmp::max(start, char_last_index[c].wrapping_add(1));
            char_last_index[c] = i as u16;
            max_len = std::cmp::max(max_len, i as u16 - start + 1);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::length_of_longest_substring("ab".to_string()), 2);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::length_of_longest_substring("abb".to_string()), 2);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::length_of_longest_substring("aabb".to_string()), 2);
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::length_of_longest_substring("abab".to_string()), 2);
    }

    #[test]
    fn myex9() {
        assert_eq!(
            Solution::length_of_longest_substring("aaabccc".to_string()),
            3
        );
    }
}
