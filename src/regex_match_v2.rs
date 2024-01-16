// https://leetcode.com/problems/regular-expression-matching/

pub struct Solution {}

impl Solution {
    fn minimize_pattern(mut pat: Vec<u8>) -> Vec<u8> {
        if pat[0] == b'*' {
            panic!("Invalid pattern: cannot start with *");
        }

        // Any case of something like a,b,c,x,*,x,a,b,c we want to change into a,b,c,x,x,*,a,b,c
        // That is, move the *s to the right as far as possible, and then remove any duplicates
        let mut i = 1;
        while i < pat.len() - 1 {
            match (pat[i - 1], pat[i], pat[i + 1], pat.get(i + 2)) {
                (b'*', b'*', _, _) => panic!("Invalid pattern: cannot have two *s in a row"),
                (_, b'*', b'.', Some(b'*')) => {
                    // a* followed by .* is equal to .*
                    pat.remove(i);
                    pat.remove(i - 1);
                }
                (b'.', b'*', _, Some(b'*')) => {
                    // .* followed by a* is equal to .*
                    pat.remove(i);
                    pat.remove(i);
                }
                (c1, b'*', c2, Some(b'*')) if c1 == c2 => {
                    // a* followed by a* is equal to just a*
                    pat.remove(i);
                    pat.remove(i);
                }
                (c1, b'*', c2, _) if c1 == c2 => {
                    // a*a is equal to aa*, but the latter is more efficient
                    pat.swap(i, i + 1);
                    i += 1;
                }
                _ => i += 1,
            }
        }
        pat
    }

    pub fn is_match(s: String, p: String) -> bool {
        let s = s.into_bytes();
        let p = Solution::minimize_pattern(p.into_bytes());

        let mut s_index = 0;
        let mut p_index = 0;

        let mut index_stack: Vec<(usize, usize)> = Vec::new();

        loop {
            match (s_index == s.len(), p_index == p.len()) {
                (true, true) => break true,
                (true, false) => {
                    if p_index + 1 < p.len() && p[p_index + 1] == b'*' {
                        p_index += 2;
                    } else {
                        break false;
                    }
                }
                (false, true) => {
                    if index_stack.is_empty() {
                        break false;
                    }
                    let t = index_stack.pop().unwrap();
                    s_index = t.0;
                    p_index = t.1;
                }
                (false, false) => match (p[p_index], p.get(p_index + 1)) {
                    (b'*', _) => {
                        panic!("Invalid pattern: cannot have * at the beginning");
                    }
                    (b'.', Some(b'*')) => {
                        index_stack.push((s_index + 1, p_index));
                        p_index += 2;
                    }
                    (b'.', _) => {
                        s_index += 1;
                        p_index += 1;
                    }
                    (x, Some(b'*')) => {
                        if s[s_index] == x {
                            index_stack.push((s_index + 1, p_index));
                        }
                        p_index += 2;
                    }
                    (x, _) => {
                        if s[s_index] == x {
                            s_index += 1;
                            p_index += 1;
                        } else {
                            if index_stack.is_empty() {
                                break false;
                            }
                            let t = index_stack.pop().unwrap();
                            s_index = t.0;
                            p_index = t.1;
                        }
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_exact_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "aa".to_string()), true);
        assert_eq!(
            Solution::is_match("aaa".to_string(), "aa".to_string()),
            false
        );
    }

    #[test]
    fn test_zero_or_more() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(
            Solution::is_match("aa".to_string(), "b*".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("ab".to_string(), "a*b*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("aba".to_string(), "a*b*".to_string()),
            false
        );
    }

    #[test]
    fn test_any_char() {
        assert_eq!(Solution::is_match("aa".to_string(), ".".to_string()), false);
        assert_eq!(Solution::is_match("ab".to_string(), "..".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), "a.".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".b".to_string()), true);
    }

    #[test]
    fn test_complicated() {
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m..*is*ip*.".to_string()),
            true
        );
    }

    #[test]
    fn test_zeroormore_edge_case() {
        assert_eq!(Solution::is_match("a".to_string(), "a.*".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "aa*".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "b*".to_string()), false);
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*a".to_string()),
            true
        );
    }

    #[test]
    fn test_failing1() {
        assert_eq!(
            Solution::is_match("a".to_string(), ".*..a*".to_string()),
            false
        );
    }

    #[test]
    fn test_failing2() {
        assert_eq!(Solution::is_match("".to_string(), "b*b*".to_string()), true);
        assert_eq!(
            Solution::is_match(
                "abcaaaaaaabaabcabac".to_string(),
                ".*ab.a.*a*a*.*".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_match(
                "abcaaaaaaabaabcabac".to_string(),
                ".*ab.a.*a*a*.*b*b*".to_string()
            ),
            true
        );
    }
}
