// https://leetcode.com/problems/shortest-completing-word/

pub struct Solution;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let (letters, letter_count) = {
            let mut letters = [0u8; 26];
            let mut letter_count = 0;
            for &b in license_plate.as_bytes() {
                if b.is_ascii_alphabetic() {
                    letters[((b | 0b100000) - b'a') as usize] += 1;
                    letter_count += 1;
                }
            }
            (letters, letter_count)
        };
        let mut shortest = None;
        'outer: for word in words.into_iter() {
            let mut letters2 = letters.clone();
            let mut lettercount2 = letter_count;
            for &b in word.as_bytes() {
                if letters2[(b - b'a') as usize] > 0 {
                    letters2[(b - b'a') as usize] -= 1;
                    lettercount2 -= 1;
                    if lettercount2 == 0 {
                        if word.len() < shortest.as_ref().map(|x: &String| x.len()).unwrap_or(16) {
                            shortest = Some(word);
                            continue 'outer;
                        }
                    }
                }
            }
        }
        shortest.expect("Answer is guaranteed.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(license_plate: &str, words: &[&str], expected: &str) {
        assert!(license_plate.len() >= 1);
        assert!(license_plate.len() <= 7);
        for &b in license_plate.as_bytes() {
            assert!(b.is_ascii_alphanumeric() || b == b' ');
        }
        assert!(words.len() >= 1);
        assert!(words.len() <= 1000);
        for &word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 15);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert_eq!(
            Solution::shortest_completing_word(
                license_plate.to_string(),
                words.iter().map(|&x| x.to_string()).collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("1s3 PSt", &["step", "steps", "stripe", "stepple"], "steps")
    }

    #[test]
    fn ex2() {
        test("1s3 456", &["looks", "pest", "stew", "show"], "pest")
    }
}
