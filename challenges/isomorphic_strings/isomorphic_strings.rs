// https://leetcode.com/problems/isomorphic-strings

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn is_isomorphic(s: String, t: String) -> bool {
//         let s = s.as_bytes();
//         let t = t.as_bytes();
//         assert!(s.len() == t.len());
//         assert!(s.len() <= 50_000);
//         let mut s_to_t = [0u8; 256];
//         let mut t_to_s = [0u8; 256];
//         for i in 0..s.len() {
//             let s_char = s[i] as usize;
//             let t_char = t[i] as usize;
//             if s_to_t[s_char] == 0 && t_to_s[t_char] == 0 {
//                 s_to_t[s_char] = t[i];
//                 t_to_s[t_char] = s[i];
//             } else if s_to_t[s_char] != t[i] || t_to_s[t_char] != s[i] {
//                 return false;
//             }
//         }
//         true
//     }
// }

// Attempted optimization (no change)
// impl Solution {
//     pub fn is_isomorphic(s: String, t: String) -> bool {
//         let s = s.into_bytes();
//         let t = t.into_bytes();
//         assert!(s.len() == t.len());
//         assert!(s.len() <= 50_000);
//         let mut s_to_t = [0u8; 256];
//         let mut t_to_s = [0u8; 256];
//         for (s_char, t_char) in std::iter::zip(s, t) {
//             if s_to_t[s_char as usize] == 0 && t_to_s[t_char as usize] == 0 {
//                 s_to_t[s_char as usize] = t_char;
//                 t_to_s[t_char as usize] = s_char;
//             } else if s_to_t[s_char as usize] != t_char || t_to_s[t_char as usize] != s_char {
//                 return false;
//             }
//         }
//         true
//     }
// }

// Bitfields go brrrr
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();
        assert!(s.len() == t.len());
        assert!(s.len() <= 50_000);
        let mut s_to_t = [0u8; 256];
        for (s_char, t_char) in std::iter::zip(s, t) {
            if s_to_t[s_char as usize] == 0 {
                s_to_t[s_char as usize] = t_char;
            } else if s_to_t[s_char as usize] != t_char {
                return false;
            }
        }
        let mut seen = [0u64; 4];
        for byte in s_to_t {
            if byte == 0 {
                continue;
            }
            let index = byte as usize / 64;
            let bit = byte as usize % 64;
            if seen[index] & (1 << bit) != 0 {
                return false;
            }
            seen[index] |= 1 << bit;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_result(str1: &str, str2: &str) -> bool {
        Solution::is_isomorphic(str1.to_string(), str2.to_string())
    }

    #[test]
    fn ex1() {
        assert_eq!(get_result("egg", "add"), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(get_result("foo", "bar"), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(get_result("paper", "title"), true);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(get_result("12", "\u{0067}\u{0067}"), false);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(get_result("badc", "baba"), false);
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(get_result("pear", "tile"), true);
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(get_result("BBBAAABA", "AAABBBBA"), false);
    }

    #[test]
    fn discussion_case5() {
        assert_eq!(get_result("ABC", "CAC"), false);
    }

    #[test]
    fn myex1() {
        assert_eq!(get_result("BBBAAAAB", "AAABBBBA"), true);
    }
}
