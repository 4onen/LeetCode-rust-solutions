// https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/

pub struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut bytes = s.bytes().skip_while(|&b| b == b' ');

        let mut prev: u8 = 0;
        loop {
            match bytes.next() {
                Some(b) if b.is_ascii_digit() => {
                    // Start number token
                    let mut num = (b - b'0') as u8;
                    loop {
                        match bytes.next() {
                            Some(b) if b.is_ascii_digit() => {
                                num = num * 10 + (b - b'0') as u8;
                            }
                            _ => break,
                        }
                    }
                    if num <= prev {
                        break false;
                    }
                    prev = num;
                }
                Some(_) => (),      // Skip non-digit
                None => break true, // End of string. Haven't broken the rule.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::are_numbers_ascending(
                "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
            ),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::are_numbers_ascending("hello world 5 x 5".to_string()),
            false
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::are_numbers_ascending(
                "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
            ),
            false
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::are_numbers_ascending("1 2 3 4 5 6 7 8 9 10".to_string()),
            true
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::are_numbers_ascending("1 2 3 4 5 6 7 8 9 10 9 8 7 6 5 4 3 2 1".to_string()),
            false
        );
    }
}
