// https://leetcode.com/problems/partition-labels/

pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut last = [0u16; 26];
        for i in 0..s.len() as u16 {
            let letter = s[i as usize] - b'a';
            last[letter as usize] = i;
        }
        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for i in 0..s.len() as u16 {
            let letter = s[i as usize] - b'a';
            end = end.max(last[letter as usize] as u16);
            if i >= end {
                result.push((end - start + 1) as i32);
                start = end + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: &[i32]) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 500);
        for b in s.as_bytes() {
            assert!(*b >= b'a' && *b <= b'z');
        }
        assert!(expected.len() >= 1);
        assert!(expected.len() <= 100);
        let mut sum = 0;
        for &n in expected {
            assert!(n >= 1);
            assert!(n as usize <= s.len());
            sum += n;
            assert!(sum <= s.len() as i32);
        }
        assert_eq!(sum, s.len() as i32);
        assert_eq!(Solution::partition_labels(s.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("ababcbacadefegdehijhklij", &[9, 7, 8])
    }

    #[test]
    fn ex2() {
        test("eccbbbbdec", &[10])
    }

    #[test]
    fn myex1() {
        test("a", &[1])
    }

    #[test]
    fn myex2() {
        test("aaabbb", &[3, 3])
    }

    #[test]
    fn myex2_1() {
        test("aababb", &[6])
    }

    #[test]
    fn myex3() {
        test("ababab", &[6])
    }
}
