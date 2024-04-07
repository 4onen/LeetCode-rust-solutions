// https://leetcode.com/problems/valid-parenthesis-string/

pub struct Solution;

// Copilot sol'n
// impl Solution {
//     pub fn check_valid_string(s: String) -> bool {
//         let mut low = 0;
//         let mut high = 0;
//         for c in s.chars() {
//             if c == '(' {
//                 low += 1;
//                 high += 1;
//             } else if c == ')' {
//                 low -= 1;
//                 high -= 1;
//             } else {
//                 low -= 1;
//                 high += 1;
//             }
//             if high < 0 {
//                 return false;
//             }
//             low = low.max(0);
//         }
//         low == 0
//     }
// }

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min_open = 0u8;
        let mut max_open = 0u8;
        for c in s.bytes() {
            match c {
                b'(' => {
                    min_open += 1;
                    max_open += 1;
                }
                b')' => {
                    min_open = min_open.saturating_sub(1);
                    let Some(new_max) = max_open.checked_sub(1) else {
                        return false;
                    };
                    max_open = new_max;
                }
                b'*' => {
                    min_open = min_open.saturating_sub(1);
                    max_open += 1;
                }
                _ => unreachable!("Invalid character"),
            }
        }
        min_open == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: bool) {
        assert_eq!(Solution::check_valid_string(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("()", true)
    }

    #[test]
    fn ex2() {
        test("(*)", true)
    }

    #[test]
    fn ex3() {
        test("(*))", true)
    }

    #[test]
    fn discussion_case1() {
        test("(((((*)))**", true)
    }

    #[test]
    fn discussion_case2() {
        test("***********************((((((((((((((((", false)
    }

    #[test]
    fn discussion_case3() {
        test("*", true)
    }

    #[test]
    fn discussion_case4() {
        test("(**(*()**()**((**(*)", true)
    }

    #[test]
    fn discussion_case5() {
        test("((*)(*))()*(*)****((*(*)())*()((()**(**)", true)
    }

    #[test]
    fn discussion_case6() {
        test(")(*()(**(*)())*))())())*)()()*(((*)()))(**()*)**(*", false)
    }

    #[test]
    fn discussion_case7() {
        test(
            ")))(*)**)))*)))))*)*(((()(((*())(***)**(**((()))()((*((()(((",
            false,
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            "()))))**)(()*()**)))()*)()())*(*)())**()*)))(**())))()**))*)*()**((*(*",
            false,
        )
    }

    #[test]
    fn discussion_case9() {
        test(
"*(*)(*))((*)*)))(*)())*())()(()*)*)****)())(()()*(*(*())()((())))*()****)(*(()))((*()*(**(*()*)*()",
        true)
    }
}
