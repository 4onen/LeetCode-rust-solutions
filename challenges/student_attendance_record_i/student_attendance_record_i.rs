// https://leetcode.com/problems/student-attendance-record-i/

pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a = false;
        let mut l = 0;
        for b in s.as_bytes() {
            match b {
                b'A' => {
                    if a {
                        return false;
                    } else {
                        a = true;
                    }
                    l = 0;
                }
                b'L' => {
                    l += 1;
                    if l == 3 {
                        return false;
                    }
                }
                _ => {
                    l = 0;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: bool) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 1000);
        for &b in s.as_bytes() {
            assert!(b == b'A' || b == b'L' || b == b'P');
        }
        assert_eq!(Solution::check_record(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("PPALLP", true)
    }

    #[test]
    fn ex2() {
        test("PPALLL", false)
    }
}
