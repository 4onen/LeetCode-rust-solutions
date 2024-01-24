// https://leetcode.com/problems/group-anagrams/

pub struct Solution;

impl Solution {
    pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        struct Alphabet {
            chars: [u8; 26],
        }
        impl Alphabet {
            fn new() -> Self {
                Self { chars: [0; 26] }
            }
            fn from_str(s: &str) -> Self {
                let mut alphabet = Self::new();
                for c in s.as_bytes() {
                    alphabet.chars[(c - b'a') as usize] += 1;
                }
                alphabet
            }
        }
        strs.sort_by_cached_key(|s| Alphabet::from_str(s));
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut prev = Alphabet::new();
        for s in strs {
            let alphabet = Alphabet::from_str(&s);
            if alphabet == prev {
                current.push(s);
            } else {
                if !current.is_empty() {
                    result.push(current);
                }
                current = vec![s];
                prev = alphabet;
            }
        }
        if !current.is_empty() {
            result.push(current);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = &["eat", "tea", "tan", "ate", "nat", "bat"];
        let mut expected = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        expected.iter_mut().for_each(|v| v.sort_unstable());
        expected.sort_unstable();

        let mut output =
            Solution::group_anagrams(input.into_iter().map(|s| s.to_string()).collect());
        // Sort the output
        output.iter_mut().for_each(|v| v.sort_unstable());
        output.sort_unstable();
        assert_eq!(output, expected);
    }

    #[test]
    fn ex2() {
        let input = &[""];
        let expected = vec![vec![""]];
        let output = Solution::group_anagrams(input.into_iter().map(|s| s.to_string()).collect());
        assert_eq!(output, expected);
    }

    #[test]
    fn ex3() {
        let input = &["a"];
        let expected = vec![vec!["a"]];
        let output = Solution::group_anagrams(input.into_iter().map(|s| s.to_string()).collect());
        assert_eq!(output, expected);
    }

    #[test]
    fn discussion_case1() {
        let input = &["", ""];
        let expected = vec![vec!["", ""]];
        let output = Solution::group_anagrams(input.into_iter().map(|s| s.to_string()).collect());
        assert_eq!(output, expected);
    }

    #[test]
    fn discussion_case2() {
        let input = &["", "b"];
        let expected = vec![vec![""], vec!["b"]];
        let mut output =
            Solution::group_anagrams(input.into_iter().map(|s| s.to_string()).collect());
        // Sort the output
        output.iter_mut().for_each(|v| v.sort_unstable());
        output.sort_unstable();
        assert_eq!(output, expected);
    }
}
