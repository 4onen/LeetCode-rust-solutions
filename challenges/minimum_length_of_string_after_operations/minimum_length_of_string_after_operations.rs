// https://leetcode.com/problems/minimum-length-of-string-after-operations/

pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut letters = [0u32; 26];
        for &b in s.as_bytes() {
            letters[(b - b'a') as usize] += 1;
        }
        let mut final_len = 0;
        for letter in letters {
            if letter > 0 {
                final_len += 2 - (letter & 1);
            }
        }
        final_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 200_000);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert!(expected >= 0);
        assert!(expected as usize <= s.len());
        assert_eq!(Solution::minimum_length(s.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("abaacbcbb", 5)
    }

    #[test]
    fn ex2() {
        test("aa", 2)
    }

    #[test]
    fn ex3() {
        test("aaa", 1)
    }

    #[test]
    fn discussion_case1() {
        test("ucvbutgkohgbcobqeyqwppbxqoynxeuuzouyvmydfhrprdbuzwqebwuiejoxsxdhbmuaiscalnteocghnlisxxawxgcjloevrdcj", 38)
    }

    #[test]
    fn my_extreme_ex1() {
        test(unsafe { std::str::from_utf8_unchecked(&[b'a'; 200_000]) }, 2)
    }
}
