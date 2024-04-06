// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/

pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let s = s.into_bytes();
        let mut s2 = std::vec::Vec::with_capacity(s.len());
        let mut paren_cnt = 0u32;
        for i in (0..s.len()).rev() {
            match s[i] {
                b')' => {
                    paren_cnt += 1;
                    s2.push(b')')
                }
                b'(' => match paren_cnt.checked_sub(1) {
                    Some(new_cnt) => {
                        paren_cnt = new_cnt;
                        s2.push(b'(')
                    }
                    None => {}
                }
                c => {
                    s2.push(c)
                }
            }
        }
        let s2 = s2;
        let mut s = s;
        let mut paren_cnt = 0u32;
        s.clear();
        for i in (0..s2.len()).rev() {
            match s2[i] {
                b'(' => {
                    paren_cnt += 1;
                    s.push(b'(')
                }
                b')' => match paren_cnt.checked_sub(1) {
                    Some(new_cnt) => {
                        paren_cnt = new_cnt;
                        s.push(b')')
                    }
                    None => {}
                }
                c => {
                    s.push(c)
                }
            }
        }
        unsafe { std::string::String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(input: &str, expected: &str) {
        assert_eq!(
            Solution::min_remove_to_make_valid(input.to_owned()),
            expected
        );
    }

    #[test]
    fn ex1() {
        do_test("lee(t(c)o)de)", "lee(t(c)o)de")
    }

    #[test]
    fn ex2() {
        do_test("a)b(c)d", "ab(c)d")
    }

    #[test]
    fn ex3() {
        do_test("))((","")
    }
}
