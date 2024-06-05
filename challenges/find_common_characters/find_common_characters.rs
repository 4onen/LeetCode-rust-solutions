// https://leetcode.com/problems/find-common-characters/

pub struct Solution;

fn count_char(word: &str) -> [u8; 26] {
    let mut count = [0; 26];
    for c in word.bytes() {
        count[(c - b'a') as usize] += 1;
    }
    count
}
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut words = words.into_iter();
        let mut counts = count_char(&words.next().unwrap());
        for word in words {
            let word_count = count_char(&word);
            for i in 0..26 {
                counts[i] = counts[i].min(word_count[i]);
            }
        }
        let mut result = Vec::new();
        for (i, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                result.push(
                    unsafe {
                        std::str::from_utf8_unchecked(std::slice::from_ref(&(i as u8 + b'a')))
                    }
                    .to_string(),
                );
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], expected: &[&str]) {
        let words = words.iter().map(|s| s.to_string()).collect();
        let mut result = Solution::common_chars(words);
        result.sort_unstable();
        let mut expected = expected
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(&["bella", "label", "roller"], &["e", "l", "l"]);
    }

    #[test]
    fn ex2() {
        test(&["cool", "lock", "cook"], &["c", "o"]);
    }
}
