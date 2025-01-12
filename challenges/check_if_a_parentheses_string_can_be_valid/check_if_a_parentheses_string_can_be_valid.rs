// https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/

pub struct Solution;

// Initial Two-pass sol'n
// impl Solution {
//     pub fn can_be_valid(s: String, locked: String) -> bool {
//         let sb = s.as_bytes();
//         let lb = locked.as_bytes();
//         let n = sb.len() as u32;
//         assert_eq!(n as usize, lb.len());
//         if s.len() % 2 == 1 {
//             return false;
//         }
//         let mut closes = 0;
//         for i in 0..sb.len() {
//             if lb[i] == b'1' {
//                 if sb[i] == b')' {
//                     closes += 1;
//                     if 2 * closes - 1 > i {
//                         return false;
//                     }
//                 }
//             }
//         }
//         let mut opens = 0;
//         for i in 0..sb.len() {
//             // Counting up positions from the end
//             if lb[sb.len() - 1 - i] == b'1' {
//                 if sb[sb.len() - 1 - i] == b'(' {
//                     opens += 1;
//                     if 2 * opens - 1 > i {
//                         return false;
//                     }
//                 }
//             }
//         }
//         true
//     }
// }

// Dtype-compressed two-pass sol'n
// impl Solution {
//     pub fn can_be_valid(s: String, locked: String) -> bool {
//         let sb = s.as_bytes();
//         let lb = locked.as_bytes();
//         let n = sb.len() as u32;
//         assert_eq!(n as usize, lb.len());
//         if s.len() % 2 == 1 {
//             return false;
//         }
//         let mut closes = 0;
//         for i in 0..n {
//             if lb[i as usize] == b'1' {
//                 if sb[i as usize] == b')' {
//                     closes += 1;
//                     if 2 * closes - 1 > i {
//                         return false;
//                     }
//                 }
//             }
//         }
//         let mut opens = 0;
//         for i in 0..n {
//             // Counting up positions from the end
//             if lb[(n - 1 - i) as usize] == b'1' {
//                 if sb[(n - 1 - i) as usize] == b'(' {
//                     opens += 1;
//                     if 2 * opens - 1 > i {
//                         return false;
//                     }
//                 }
//             }
//         }
//         true
//     }
// }

// One-pass two-Stack sol'n
// impl Solution {
//     pub fn can_be_valid(s: String, locked: String) -> bool {
//         let sb = s.as_bytes();
//         let lb = locked.as_bytes();
//         let n = sb.len() as u32;
//         assert_eq!(n as usize, lb.len());
//         if s.len() % 2 == 1 {
//             return false;
//         }
//         let mut locked_open = vec![];
//         let mut switchable = vec![];
//         for i in 0..n {
//             if lb[i as usize] == b'0' {
//                 switchable.push(i);
//             } else if sb[i as usize] == b'(' {
//                 locked_open.push(i);
//             } else {
//                 match (locked_open.last(), switchable.last()) {
//                     (Some(&_), _) => {
//                         locked_open.pop();
//                     }
//                     (None, Some(&_)) => {
//                         switchable.pop();
//                     }
//                     _ => return false,
//                 }
//             }
//         }
//         loop {
//             let Some(lo) = locked_open.pop() else {
//                 break true;
//             };
//             let Some(swi) = switchable.pop() else {
//                 break false;
//             };
//             if lo > swi {
//                 break false;
//             }
//         }
//     }
// }

// Optimized away stack element fetch one-pass two-stack
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let sb = s.as_bytes();
        let lb = locked.as_bytes();
        let n = sb.len() as u32;
        assert_eq!(n as usize, lb.len());
        if s.len() % 2 == 1 {
            return false;
        }
        let mut locked_open = vec![];
        let mut switchable = vec![];
        for i in 0..n {
            if lb[i as usize] == b'0' {
                switchable.push(i);
            } else if sb[i as usize] == b'(' {
                locked_open.push(i);
            } else {
                if locked_open.len() > 0 {
                    locked_open.pop();
                } else if switchable.len() > 0 {
                    switchable.pop();
                } else {
                    return false;
                }
            }
        }
        loop {
            let Some(lo) = locked_open.pop() else {
                break true;
            };
            let Some(swi) = switchable.pop() else {
                break false;
            };
            if lo > swi {
                break false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, locked: &str, expected: bool) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100_000);
        assert!(locked.len() >= 1);
        assert!(locked.len() <= 100_000);
        if s.len() % 2 == 1 {
            assert_eq!(expected, false);
        }
        for &b in s.as_bytes() {
            assert!(b == b'(' || b == b')')
        }
        for &b in locked.as_bytes() {
            assert!(b == b'1' || b == b'0')
        }
        assert_eq!(
            Solution::can_be_valid(s.to_owned(), locked.to_owned()),
            expected
        );
    }

    fn test_all_locked(s: &str, expected: bool) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 30); // Dont' try any cases that are too excessive.
        if s.len() % 2 == 1 {
            assert_eq!(expected, false);
        }
        for &b in s.as_bytes() {
            assert!(b == b'(' || b == b')')
        }
        let mut lock = [b'0'; 30];
        loop {
            assert_eq!(
                Solution::can_be_valid(
                    s.to_owned(),
                    unsafe { std::str::from_utf8_unchecked(&lock[..s.len()]) }.to_owned(),
                ),
                expected
            );
            let mut carry = 1;
            for i in 0..s.len() {
                (lock[i], carry) = (lock[i] ^ carry, lock[i] & carry)
            }
            if carry > 0 {
                break;
            }
        }
    }

    #[test]
    fn ex1() {
        test("))()))", "010100", true)
    }

    #[test]
    fn ex2() {
        test_all_locked("()()", true)
    }

    #[test]
    fn ex3() {
        test(")", "0", false)
    }

    #[test]
    fn discussion_case1() {
        test(")(())(", "010010", true)
    }

    #[test]
    fn discussion_case2() {
        test_all_locked("(()())", true)
    }

    #[test]
    fn discussion_case3() {
        test_all_locked("(()))", false)
    }

    #[test]
    fn discussion_case3_1() {
        test_all_locked("((())", false)
    }

    #[test]
    fn discussion_case4() {
        test(
            "((()(()()))()((()()))))(((()(()",
            "1011110010010100111010000001001",
            false,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            "())(()(()(())()())(())((())(()())((())))))(((((((())(()))))(",
            "100011110110011011010111100111011101111110000101001101001111",
            false,
        )
    }

    #[test]
    fn discussion_case5_1() {
        test(
            "())((()(())())(())((())(()))))(((((((())(()))))(",
            "100011011000110101111001011110000101001101001111",
            false,
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            "())()))()(()(((())(()()))))((((()())(())",
            "1011101100010001001011000000110010100101",
            true,
        )
    }

    #[test]
    fn discussion_case7() {
        test(")(", "00", true)
    }

    #[test]
    fn discussion_case8() {
        test("()))", "0010", true)
    }

    #[test]
    fn myex1() {
        test("()())))(()", "0010001000", true)
    }

    #[test]
    fn myex2() {
        test("()())))(())(", "001000100001", false)
    }
}
