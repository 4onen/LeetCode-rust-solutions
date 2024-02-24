// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn keyboard(digit: u8) -> &'static [u8] {
            match digit {
                b'2' => &[b'a', b'b', b'c'],
                b'3' => &[b'd', b'e', b'f'],
                b'4' => &[b'g', b'h', b'i'],
                b'5' => &[b'j', b'k', b'l'],
                b'6' => &[b'm', b'n', b'o'],
                b'7' => &[b'p', b'q', b'r', b's'],
                b'8' => &[b't', b'u', b'v'],
                b'9' => &[b'w', b'x', b'y', b'z'],
                _ => unreachable!(),
            }
        }
        let mut result = vec![];
        for digit in digits.bytes() {
            let letters = keyboard(digit);
            if result.is_empty() {
                result = letters.iter().map(|&c| vec![c]).collect();
            } else {
                let mut new_result = vec![];
                for &letter in letters {
                    for r in result.iter() {
                        let mut new_val = std::vec::Vec::with_capacity(r.len() + 1);
                        new_val.extend_from_slice(r);
                        new_val.push(letter);
                        new_result.push(new_val);
                    }
                }
                result = new_result;
            }
        }
        result
            .into_iter()
            .map(|v| String::from_utf8(v).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let digits = "23".to_string();
        let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        let mut result = Solution::letter_combinations(digits);
        result.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex2() {
        let digits = "".to_string();
        let expected: Vec<String> = vec![];
        let mut result = Solution::letter_combinations(digits);
        result.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex3() {
        let digits = "2".to_string();
        let expected = vec!["a", "b", "c"];
        let mut result = Solution::letter_combinations(digits);
        result.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn discussion_case1() {
        let digits = "234".to_string();
        let expected = vec![
            "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi",
            "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei",
            "cfg", "cfh", "cfi",
        ];
        let mut result = Solution::letter_combinations(digits);
        result.sort_unstable();
        assert_eq!(result, expected);
    }
}
