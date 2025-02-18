// https://leetcode.com/problems/construct-smallest-number-from-di-string/

pub struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let pattern = pattern.as_bytes();
        let mut d_following_counts = [0u8; 9];
        let mut last_i = 0;
        for i in 0..pattern.len() as u8 {
            match pattern[i as usize] {
                b'I' => {
                    last_i = i + 1;
                }
                b'D' => {
                    d_following_counts[last_i as usize] += 1;
                }
                _ => unreachable!(),
            }
        }
        let mut available = [true; 9];
        let mut result = std::vec::Vec::new();
        let mut n = d_following_counts[0];
        available[n as usize] = false;
        result.push(b'1' + n);
        if n > 0 {
            n -= 1;
        }
        for i in 0..pattern.len() as u8 {
            if pattern[i as usize] == b'I' {
                // Find the smallest available digit, then add
                // the d_following_counts[i+1] to that n.
                let mut smallest = 10;
                for j in 0..available.len() as u8 {
                    if available[j as usize] {
                        smallest = j;
                        break;
                    }
                }
                n = smallest + d_following_counts[1 + i as usize];
            }
            available[n as usize] = false;
            result.push(b'1' + n);
            if n > 0 {
                n -= 1;
            }
        }
        unsafe { std::string::String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(pattern: &str, expected: &str) {
        assert!(pattern.len() >= 1);
        assert!(pattern.len() <= 8);
        assert_eq!(pattern.len() + 1, expected.len());
        assert_eq!(Solution::smallest_number(pattern.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("IIIDIDDD", "123549876")
    }

    #[test]
    fn ex2() {
        test("DDD", "4321")
    }

    #[test]
    fn discussion_case1() {
        test("IDI", "1324")
    }

    #[test]
    fn discussion_case2() {
        test("DIDDD", "216543")
    }
}
