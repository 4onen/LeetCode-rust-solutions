// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let mut n = n as u8;
        let mut result = std::vec::Vec::with_capacity(n as usize);
        result.push(if k > 3 << (n - 1) {
            return "".to_string();
        } else if k > 1 << n {
            k -= 1 << n;
            b'c'
        } else if k > 1 << (n - 1) {
            k -= 1 << (n - 1);
            b'b'
        } else {
            b'a'
        });
        loop {
            n -= 1;
            if n <= 0 {
                break;
            }
            result.push(if k > 1 << (n - 1) {
                k -= 1 << (n - 1);
                match result.last().copied().unwrap() {
                    b'a' => b'c',
                    b'b' => b'c',
                    b'c' => b'b',
                    _ => unreachable!(),
                }
            } else {
                match result.last().copied().unwrap() {
                    b'a' => b'b',
                    b'b' => b'a',
                    b'c' => b'a',
                    _ => unreachable!(),
                }
            })
        }
        unsafe { std::string::String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u8, k: u8, expected: &str) {
        assert!(n >= 1);
        assert!(n <= 10);
        assert!(k >= 1);
        assert!(k <= 100);
        if expected.len() > 0 {
            assert_eq!(expected.len(), n as usize);
        }
        let mut last = b'0';
        for &b in expected.as_bytes() {
            assert!(b == b'a' || b == b'b' || b == b'c');
            assert_ne!(b, last);
            last = b;
        }
        assert_eq!(Solution::get_happy_string(n as i32, k as i32), expected);
    }

    #[test]
    fn ex1() {
        test(1, 3, "c")
    }

    #[test]
    fn ex1_1() {
        test(1, 1, "a")
    }

    #[test]
    fn ex1_2() {
        test(1, 2, "b")
    }

    #[test]
    fn ex2() {
        test(1, 4, "")
    }

    #[test]
    fn ex3() {
        test(3, 9, "cab")
    }

    #[test]
    fn ex3_1() {
        test(3, 1, "aba")
    }

    #[test]
    fn ex3_2() {
        test(3, 2, "abc")
    }

    #[test]
    fn ex3_3() {
        test(3, 3, "aca")
    }

    #[test]
    fn ex3_4() {
        test(3, 4, "acb")
    }

    #[test]
    fn ex3_5() {
        test(3, 5, "bab")
    }

    #[test]
    fn ex3_6() {
        test(3, 6, "bac")
    }

    #[test]
    fn ex3_7() {
        test(3, 7, "bca")
    }

    #[test]
    fn ex3_8() {
        test(3, 8, "bcb")
    }

    #[test]
    fn ex3_10() {
        test(3, 10, "cac")
    }

    #[test]
    fn ex3_11() {
        test(3, 11, "cba")
    }

    #[test]
    fn ex3_12() {
        test(3, 12, "cbc")
    }

    #[test]
    fn ex3_13() {
        test(3, 13, "")
    }

    #[test]
    fn myex2_1() {
        test(2, 1, "ab")
    }

    #[test]
    fn myex2_2() {
        test(2, 2, "ac")
    }

    #[test]
    fn myex2_3() {
        test(2, 3, "ba")
    }

    #[test]
    fn myex2_4() {
        test(2, 4, "bc")
    }

    #[test]
    fn myex2_5() {
        test(2, 5, "ca")
    }

    #[test]
    fn myex2_6() {
        test(2, 6, "cb")
    }

    #[test]
    fn myex2_7() {
        test(2, 7, "")
    }
}
