// https://leetcode.com/problems/wildcard-matching/

pub struct Solution;

// Too slow
// impl Solution {
//     pub fn is_match(s: String, p: String) -> bool {
//         let s_bytes = s.as_bytes();
//         let p_bytes = p.as_bytes();
//         let mut stack: Vec<(u16, u16)> = vec![(0, 0)];
//         while let Some((mut s_idx, mut p_idx)) = stack.pop() {
//             println!("{} {} {}", stack.len(), s_idx, p_idx);
//             let s_exhausted = s_idx >= s_bytes.len() as u16;
//             let p_exhausted = p_idx >= p_bytes.len() as u16;
//             if s_exhausted && p_exhausted {
//                 return true;
//             }
//             if p_exhausted {
//                 // We have exhausted the pattern, but not the string.
//                 // No match.
//                 continue;
//             }
//             if s_exhausted {
//                 // We have exhausted the string, but not the pattern.
//                 // If all remaining characters in the pattern are '*',
//                 // then we have a match.
//                 if p_bytes[p_idx as usize..].iter().all(|&c| c == b'*') {
//                     return true;
//                 }
//                 // Otherwise, no match.
//                 continue;
//             }
//             // We have not exhausted either the string or the pattern.
//             let c = p_bytes[p_idx as usize];
//             if c == b'*' {
//                 // Skip over all consecutive '*' characters.
//                 while p_idx < p_bytes.len() as u16 && p_bytes[p_idx as usize] == b'*' {
//                     p_idx += 1;
//                 }
//                 // Explore both if we eat a character with this asterisk
//                 // and if we don't.
//                 stack.push((s_idx, p_idx));
//                 stack.push((s_idx + 1, p_idx - 1));
//                 continue;
//             }
//             if c == b'?' || c == s_bytes[s_idx as usize] {
//                 while s_idx < s_bytes.len() as u16
//                     && p_idx < p_bytes.len() as u16
//                     && (p_bytes[p_idx as usize] == b'?'
//                         || p_bytes[p_idx as usize] == s_bytes[s_idx as usize])
//                 {
//                     // If we have a match, then advance both pointers.
//                     s_idx += 1;
//                     p_idx += 1;
//                 }
//                 // Now we have either exhausted the string or the pattern,
//                 // or we ran into an asterisk,
//                 // or we ran into a mismatch.
//                 stack.push((s_idx, p_idx));
//             }
//             // If we get here, then no match this path.
//             // Backtrack to the next possible path in the stack.
//         }
//         // If we get here, then no match.
//         false
//     }
// }

// Passing sol'n (Logic is same as above but with faster constructs)
// impl Solution {
//     pub fn is_match(s: String, p: String) -> bool {
//         let s = s.into_bytes();
//         let p = p.into_bytes();
//         let mut s_idx = 0;
//         let mut p_idx = 0;
//         let mut index_stack: Vec<(u16, u16)> = vec![];
//         loop {
//             match (s_idx == s.len(), p_idx == p.len()) {
//                 (true, true) => break true,
//                 (true, false) => {
//                     if p[p_idx] == b'*' {
//                         p_idx += 1;
//                     } else {
//                         break false;
//                     }
//                 }
//                 (false, true) => {
//                     if index_stack.is_empty() {
//                         break false;
//                     }
//                     let t = index_stack.pop().unwrap();
//                     s_idx = t.0 as usize;
//                     p_idx = t.1 as usize;
//                 }
//                 (false, false) => match p[p_idx] {
//                     b'*' => {
//                         while let Some(b'*') = p.get(p_idx) {
//                             p_idx += 1;
//                         }
//                         index_stack.push((s_idx as u16 + 1, p_idx as u16 - 1));
//                     }
//                     b'?' => {
//                         s_idx += 1;
//                         p_idx += 1;
//                     }
//                     c if c == s[s_idx] => {
//                         s_idx += 1;
//                         p_idx += 1;
//                     }
//                     _ => {
//                         if index_stack.is_empty() {
//                             break false;
//                         }
//                         let t = index_stack.pop().unwrap();
//                         s_idx = t.0 as usize;
//                         p_idx = t.1 as usize;
//                     }
//                 },
//             }
//         }
//     }
// }

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.into_bytes();
        let p = p.into_bytes();
        let mut s_idx = 0;
        let mut p_idx = 0;
        let mut star_cache: Option<(u16, u16)> = None;
        loop {
            match (s_idx == s.len(), p_idx == p.len()) {
                (true, true) => break true,
                (true, false) => {
                    if p[p_idx] == b'*' {
                        p_idx += 1;
                    } else {
                        break false;
                    }
                }
                (false, true) => {
                    if let Some(t) = star_cache {
                        s_idx = t.0 as usize;
                        p_idx = t.1 as usize;
                    } else {
                        break false;
                    }
                }
                (false, false) => match p[p_idx] {
                    b'*' => {
                        while let Some(b'*') = p.get(p_idx) {
                            p_idx += 1;
                        }
                        star_cache = Some((s_idx as u16 + 1, p_idx as u16 - 1));
                    }
                    b'?' => {
                        s_idx += 1;
                        p_idx += 1;
                    }
                    c if c == s[s_idx] => {
                        s_idx += 1;
                        p_idx += 1;
                    }
                    _ => {
                        if let Some(t) = star_cache {
                            s_idx = t.0 as usize;
                            p_idx = t.1 as usize;
                        } else {
                            break false;
                        }
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::is_match("cb".to_string(), "?a".to_string()),
            false
        );
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(
            Solution::is_match("abcabczzzde".to_string(), "*abc???de".to_string()),
            true
        );
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::is_match("adceb".to_string(), "*a*b".to_string()),
            true
        );
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
            false
        );
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(
            Solution::is_match(
                "aaabababaaabaababbbaaaabbbbbbabbbbabbbabbaabbababab".to_string(),
                "*ab**ba**b*b*aaab*b".to_string()
            ),
            true
        );
    }

    #[test]
    fn discussion_case5() {
        assert_eq!(
            Solution::is_match("abcd".to_string(), "*z".to_string()),
            false
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m??*ss*?i*pi".to_string()),
            false
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m********pi".to_string()),
            true
        );
    }

    #[test]
    fn myex3() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m*********i".to_string()),
            true
        );
    }

    #[test]
    fn myex4() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m*********".to_string()),
            true
        );
    }

    #[test]
    fn myex5() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m*********?".to_string()),
            true
        );
    }

    #[test]
    fn myex6() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m*********?*".to_string()),
            true
        );
    }

    #[test]
    fn myex7() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m*********?*?".to_string()),
            true
        );
    }

    #[test]
    fn myex8() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "**********?*?*".to_string()),
            true
        );
    }

    #[test]
    fn myex9() {
        assert_eq!(Solution::is_match("".to_string(), "*".to_string()), true)
    }

    #[test]
    fn time_limit_exceeded() {
        assert_eq!(
            Solution::is_match(
                "abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_string(),
                "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb".to_string(),
            ),
            false
        );
    }
}
