// https://leetcode.com/problems/largest-substring-between-two-equal-characters/

pub struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let bytes = s.into_bytes();
        (0..bytes.len())
            .rev()
            .map(|i: usize| -> i32 {
                for j in 0..i {
                    if bytes[i] == bytes[j] {
                        return (i - j - 1) as i32;
                    }
                }
                -1
            })
            .max()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::max_length_between_equal_characters("aa".to_string()),
            0
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::max_length_between_equal_characters("abca".to_string()),
            2
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".to_string()),
            -1
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::max_length_between_equal_characters("cabbac".to_string()),
            4
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::max_length_between_equal_characters("caczbab".to_string()),
            3
        );
    }
}
