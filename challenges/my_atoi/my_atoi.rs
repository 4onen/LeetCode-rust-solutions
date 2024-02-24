// https://leetcode.com/problems/string-to-integer-atoi/

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut bytes = s.bytes().skip_while(|&b| b == b' ');
        let mut result = 0;
        let mut is_positive = true;
        match bytes.next() {
            Some(b'-') => is_positive = false,
            Some(b'+') => (),
            Some(b) if b.is_ascii_digit() => result = b as i32 - b'0' as i32,
            _ => return 0,
        }

        let number_bytes = bytes.take_while(|&b| b.is_ascii_digit());
        if is_positive {
            for b in number_bytes {
                match result
                    .checked_mul(10)
                    .and_then(|r| r.checked_add((b - b'0') as i32))
                {
                    Some(r) => result = r,
                    None => return i32::MAX,
                }
            }
        } else {
            for b in number_bytes {
                match result
                    .checked_mul(10)
                    .and_then(|r| r.checked_sub((b - b'0') as i32))
                {
                    Some(r) => result = r,
                    None => return i32::MIN,
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }
}
