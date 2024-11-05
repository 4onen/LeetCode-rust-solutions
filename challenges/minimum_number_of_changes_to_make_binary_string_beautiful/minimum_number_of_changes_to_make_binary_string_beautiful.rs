// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_changes(s: String) -> i32 {
//         let s = s.as_bytes();
//         let mut res = 0;
//         for i in (0..s.len()).step_by(2) {
//             match s[i..i + 2] {
//                 [b'0', b'0'] | [b'1', b'1'] => {}
//                 [b'0', b'1'] | [b'1', b'0'] => {
//                     res += 1;
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         res
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;
        for i in (0..s.len()).step_by(2) {
            if s[i] != s[i + 1] {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 2);
        assert!(s.len() <= 100_000);
        assert!(s.len() % 2 == 0);
        for b in s.bytes() {
            assert!(b == b'0' || b == b'1');
        }
        assert_eq!(Solution::min_changes(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("1001", 2)
    }

    #[test]
    fn ex2() {
        test("10", 1)
    }

    #[test]
    fn ex3() {
        test("0000", 0)
    }

    #[test]
    fn discussion_case1() {
        test("0011", 0)
    }

    #[test]
    fn discussion_case2() {
        test("111100", 0)
    }

    #[test]
    fn discussion_case3() {
        test("111000", 1)
    }

    #[test]
    fn discussion_case4() {
        test("11001100", 0)
    }
}
