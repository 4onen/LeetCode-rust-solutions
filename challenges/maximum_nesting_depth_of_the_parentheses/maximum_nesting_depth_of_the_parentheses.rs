// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/

pub struct Solution;

// impl Solution {
//     pub fn max_depth(s: String) -> i32 {
//         let s = s.as_bytes();
//         let mut max_depth = 0u8;
//         let mut depth = 0u8;
//         let mut i = 0;
//         while i < s.len() {
//             match s[i] {
//                 b'(' => {
//                     depth += 1;
//                     max_depth = std::cmp::max(max_depth, depth);
//                 }
//                 b')' => {
//                     depth -= 1;
//                 }
//                 _ => {}
//             }
//             i += 1;
//         }
//         max_depth as i32
//     }
// }

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;
        for c in s.bytes() {
            match c {
                b'(' => {
                    depth += 1;
                    max_depth = std::cmp::max(max_depth, depth);
                }
                b')' => {
                    depth -= 1;
                }
                _ => {}
            }
        }
        max_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(expr: &str, expected: i32) {
        assert_eq!(Solution::max_depth(expr.to_string()), expected);
    }

    #[test]
    fn ex1() {
        do_test("(1+(2*3)+((8)/4))+1", 3);
    }

    #[test]
    fn ex2() {
        do_test("(1)+((2))+(((3)))", 3);
    }
}
