// https://leetcode.com/problems/optimal-partition-of-string/

pub struct Solution;

// Greedy sol'n
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut seen = [false; 26];
        let mut res = 1;
        for &b in s {
            let i = b - b'a';
            if seen[i as usize] {
                res += 1;
                seen.fill(false);
            }
            seen[i as usize] = true;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100_000);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert!(expected >= 1);
        assert!(expected <= s.len() as i32);
        assert_eq!(Solution::partition_string(s.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test(
            "abacaba",
            4,
        )
    }

    #[test]
    fn ex2() {
        test(
            "ssssss",
            6,
        )
    }
}
