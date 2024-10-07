// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let s = s.as_bytes();
        // Problem say this should never realloc:
        let mut stack = std::vec::Vec::with_capacity(100);
        for &b in s {
            match (stack.last(), b) {
                (Some(&b'A'), b'B') => {
                    stack.pop();
                }
                (Some(&b'C'), b'D') => {
                    stack.pop();
                }
                (_, b) => stack.push(b),
            }
        }
        stack.len() as i32
    }
}

// Stack-alloc sol'n
// impl Solution {
//     pub fn min_length(s: String) -> i32 {
//         let s = s.as_bytes();
//         #[allow(invalid_value)]
//         let mut stack: [u8; 100] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//         let mut stack_next = 0u8;
//         for &b in s {
//             if stack_next > 0 {
//                 match (stack[stack_next as usize - 1], b) {
//                     (b'A', b'B') => stack_next -= 1,
//                     (b'C', b'D') => stack_next -= 1,
//                     _ => {
//                         stack[stack_next as usize] = b;
//                         stack_next += 1;
//                     }
//                 }
//             } else {
//                 stack[stack_next as usize] = b;
//                 stack_next += 1;
//             }
//         }
//         stack_next as i32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: u8) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100);
        for b in s.bytes() {
            assert!(b >= b'A' && b <= b'Z');
        }
        assert_eq!(Solution::min_length(s.to_string()), expected as i32);
    }

    #[test]
    fn ex1() {
        test("ABFCACDB", 2)
    }

    #[test]
    fn ex2() {
        test("ACBBD", 5)
    }
}
