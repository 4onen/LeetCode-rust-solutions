// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/

pub struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![];
        let mut current = vec![];
        for b in s.bytes() {
            match b {
                b'(' => {
                    stack.push(current);
                    current = vec![];
                }
                b')' => {
                    let mut old_current = stack.pop().unwrap();
                    old_current.extend(current.into_iter().rev());
                    current = old_current;
                }
                _ => {
                    current.push(b);
                }
            }
        }
        unsafe { String::from_utf8_unchecked(current) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: &str) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 2000);
        assert!(expected.len() <= s.len());
        let mut paren_count = 0;
        for b in s.bytes() {
            if b == b'(' {
                paren_count += 1;
            } else if b == b')' {
                paren_count -= 1;
            }
            assert!(b >= b'a' && b <= b'z' || b == b'(' || b == b')');
        }
        assert_eq!(paren_count, 0);
        for b in expected.bytes() {
            assert!(b >= b'a' && b <= b'z');
        }
        assert_eq!(Solution::reverse_parentheses(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("(abcd)", "dcba");
    }

    #[test]
    fn ex2() {
        test("(u(love)i)", "iloveu");
    }

    #[test]
    fn ex3() {
        test("(ed(et(oc))el)", "leetcode");
    }
}
