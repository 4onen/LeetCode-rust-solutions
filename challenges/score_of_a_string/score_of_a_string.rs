// https://leetcode.com/problems/score-of-a-string/

pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut bytes = s.bytes();
        let mut last = bytes.next().unwrap() as i8;
        let mut acc = 0;
        for byte in bytes {
            acc += (byte as i8 - last).abs() as i32;
            last = byte as i8;
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, exp: i32) {
        assert_eq!(Solution::score_of_string(s.to_string()), exp);
    }

    #[test]
    fn ex1() {
        test("hello", 13);
    }

    #[test]
    fn ex2() {
        test("zaz", 50);
    }
}
