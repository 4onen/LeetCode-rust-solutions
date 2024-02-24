// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/

pub struct Solution;

// Easy iterative solution
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count = 0;
        let mut prev = b'0';
        let bytes = s.as_bytes();
        for b in bytes {
            if *b == prev {
                count += 1;
            }
            prev = if prev == b'0' { b'1' } else { b'0' };
        }
        std::cmp::min(count, s.len() - count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::min_operations("0100".to_string()), 1);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::min_operations("10".to_string()), 0);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::min_operations("1111".to_string()), 2);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::min_operations("001010".to_string()), 1);
    }
}
