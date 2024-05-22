// https://leetcode.com/problems/palindrome-partitioning/

pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &[u8]) -> bool {
            assert!(s.len() > 0);
            let mut i = 0;
            let mut j = s.len() - 1;
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }
        fn to_string(s: &[u8]) -> String {
            unsafe { std::str::from_utf8_unchecked(s) }.to_string()
        }
        fn dfs(s: &[u8], start: u8, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
            if start == s.len() as u8 {
                result.push(path.clone());
                return;
            }
            for end in start + 1..=s.len() as u8 {
                let sub = &s[start as usize..end as usize];
                if is_palindrome(sub) {
                    path.push(to_string(sub));
                    dfs(s, end, path, result);
                    path.pop();
                }
            }
        }
        let mut result = Vec::new();
        dfs(s.as_bytes(), 0, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, output: &[&[&str]]) {
        assert!(input.len() > 0);
        assert!(input.len() <= 16);
        let mut expected = output.to_vec();
        let mut result = Solution::partition(input.to_string());
        assert_eq!(result.len(), output.len());
        expected.sort_unstable();
        result.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test("aab", &[&["a", "a", "b"], &["aa", "b"]]);
    }

    #[test]
    fn ex2() {
        test("a", &[&["a"]]);
    }

    #[test]
    fn discussion_case1() {
        test(
            "aaab",
            &[
                &["a", "a", "a", "b"],
                &["a", "aa", "b"],
                &["aa", "a", "b"],
                &["aaa", "b"],
            ],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            "abcaa",
            &[&["a", "b", "c", "a", "a"], &["a", "b", "c", "aa"]],
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            "abbab",
            &[
                &["a", "b", "b", "a", "b"],
                &["a", "b", "bab"],
                &["a", "bb", "a", "b"],
                &["abba", "b"],
            ],
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            "abaca",
            &[
                &["a", "b", "a", "c", "a"],
                &["a", "b", "aca"],
                &["aba", "c", "a"],
            ],
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            "aaa",
            &[&["a", "a", "a"], &["a", "aa"], &["aa", "a"], &["aaa"]],
        )
    }

    #[test]
    fn discussion_case6() {
        test("bc", &[&["b", "c"]])
    }

    #[test]
    fn discussion_case7() {
        test(
            "abcdefghijklmnop",
            &[&[
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
            ]],
        )
    }
}
