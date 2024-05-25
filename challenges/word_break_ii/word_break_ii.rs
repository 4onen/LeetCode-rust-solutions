// https://leetcode.com/problems/word-break-ii/

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn backtrack<'a>(
            s: &'a str,
            word_dict: &std::collections::HashSet<String>,
            cache: &mut std::collections::HashMap<&'a str, Vec<String>>,
        ) -> Vec<String> {
            if let Some(v) = cache.get(s) {
                return v.clone();
            }
            let mut result = Vec::new();
            for word in word_dict.iter() {
                if s.starts_with(word) {
                    if s.len() == word.len() {
                        result.push(word.clone());
                    } else {
                        for r in backtrack(&s[word.len()..], word_dict, cache) {
                            result.push(format!("{} {}", word, r));
                        }
                    }
                }
            }
            cache.insert(s, result.clone());
            result
        }
        backtrack(
            &s,
            &word_dict.into_iter().collect(),
            &mut std::collections::HashMap::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, word_dict: &[&str], expected: &[&str]) {
        assert!(s.len() > 0);
        assert!(s.len() <= 20);
        assert!(s.bytes().all(|b| b.is_ascii_lowercase()));
        assert!(word_dict.len() > 0);
        assert!(word_dict.len() <= 1000);
        for (i, &word) in word_dict.into_iter().enumerate() {
            assert!(word.len() > 0);
            assert!(word.len() <= 10);
            assert!(word.bytes().all(|b| b.is_ascii_lowercase()));
            for j in 0..i {
                assert_ne!(word, word_dict[j]);
            }
        }
        assert!(expected.len() <= 100000);
        for &s in expected {
            assert!(s.len() > 0);
            assert!(s.len() <= 40);
            assert!(s.bytes().all(|b| b.is_ascii_lowercase() || b == b' '));
        }
        let word_dict = word_dict.iter().map(|s| s.to_string()).collect();
        let mut result = Solution::word_break(s.to_string(), word_dict);
        result.sort_unstable();
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(
            "catsanddog",
            &["cat", "cats", "and", "sand", "dog"],
            &["cats and dog", "cat sand dog"],
        )
    }

    #[test]
    fn ex2() {
        test(
            "pineapplepenapple",
            &["apple", "pen", "applepen", "pine", "pineapple"],
            &[
                "pine apple pen apple",
                "pineapple pen apple",
                "pine applepen apple",
            ],
        )
    }

    #[test]
    fn ex3() {
        test("catsandog", &["cats", "dog", "sand", "and", "cat"], &[])
    }

    #[test]
    fn discussion_case1() {
        test("catsandog", &["cats", "dog", "sand", "and", "cat"], &[])
    }

    #[test]
    fn discussion_case2() {
        test("a", &["b"], &[])
    }

    #[test]
    fn discussion_case3() {
        test(
            "aaaaaa",
            &["a", "aa", "aaa"],
            &[
                "a a a a a a",
                "a a a a aa",
                "a a a aa a",
                "a a aa a a",
                "a aa a a a",
                "aa a a a a",
                "a a aa aa",
                "a aa a aa",
                "aa a a aa",
                "a aa aa a",
                "aa a aa a",
                "aa aa a a",
                "aa aa aa",
                "a a a aaa",
                "a aa aaa",
                "aa a aaa",
                "a a aaa a",
                "aa aaa a",
                "a aaa a a",
                "a aaa aa",
                "aaa a a a",
                "aaa a aa",
                "aaa aa a",
                "aaa aaa",
            ],
        )
    }

    #[test]
    fn myex1() {
        test("a", &["a"], &["a"])
    }
}
