// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

pub struct Solution;

// impl Solution {
//     pub fn min_add_to_make_valid(s: String) -> i32 {
//         let s = s.into_bytes();
//         let mut stack = Vec::with_capacity(s.len());
//         for b in s {
//             match b {
//                 b'(' => stack.push(b'('),
//                 b')' => {
//                     if stack.last() == Some(&b'(') {
//                         stack.pop();
//                     } else {
//                         stack.push(b')');
//                     }
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         stack.len() as i32
//     }
// }

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut opening = 0;
        let mut closing = 0;
        for b in s.into_bytes() {
            match b {
                b'(' => opening += 1,
                b')' if opening > 0 => opening -= 1,
                b')' => closing += 1,
                _ => unreachable!(),
            }
        }
        opening + closing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 1000);
        for b in s.bytes() {
            assert!(b == b'(' || b == b')');
        }
        assert_eq!(Solution::min_add_to_make_valid(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("())", 1)
    }

    #[test]
    fn ex2() {
        test("(((", 3)
    }

    #[test]
    fn ex3() {
        test("()))", 2)
    }

    #[test]
    fn myex0() {
        test("()", 0)
    }
}
