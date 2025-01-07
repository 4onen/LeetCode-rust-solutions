// https://leetcode.com/problems/string-matching-in-an-array/

pub struct Solution;

impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by_key(|x| x.len());
        let mut i = 0;
        'outer: while i < words.len() {
            for j in i + 1..words.len() {
                if words[j].contains(&words[i]) {
                    i += 1;
                    continue 'outer;
                }
            }
            words.remove(i);
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], expected: &[&str]) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 100);
        assert!(expected.len() <= words.len());
        let mut seen = std::collections::HashSet::new();
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 30);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
            assert!(seen.insert(word));
        }
        let result = {
            let mut result =
                Solution::string_matching(words.iter().map(|&x| x.to_owned()).collect());
            result.sort_unstable();
            result
        };
        let expected_sorted = {
            let mut expected = expected.to_vec();
            expected.sort_unstable();
            expected
        };
        assert_eq!(result, expected_sorted);
    }

    #[test]
    fn ex1() {
        test(&["mass", "as", "hero", "superhero"], &["as", "hero"])
    }

    #[test]
    fn ex2() {
        test(&["leetcode", "et", "code"], &["et", "code"])
    }

    #[test]
    fn ex3() {
        test(&["blue", "green", "bu"], &[])
    }
}
