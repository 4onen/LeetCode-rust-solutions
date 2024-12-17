// https://leetcode.com/problems/construct-string-with-repeat-limit/

pub struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut counts = [0u32; 26];
        let mut s = s.into_bytes();
        for b in s.drain(..) {
            counts[(b - b'a') as usize] += 1;
        }
        let mut letter_idx = b'z' - b'a';
        let mut repeat_cnt = 0;
        'outer: loop {
            if counts[letter_idx as usize] < 1 {
                if letter_idx < 1 {
                    break;
                }
                letter_idx -= 1;
                repeat_cnt = 0;
            } else if repeat_cnt >= repeat_limit {
                // Search for another nonzero char
                for backup_letter_idx in (0..letter_idx).rev() {
                    if counts[backup_letter_idx as usize] > 0 {
                        counts[backup_letter_idx as usize] -= 1;
                        s.push(backup_letter_idx + b'a');
                        repeat_cnt = 0;
                        continue 'outer;
                    }
                }
                break; // Couldn't find one
            } else {
                counts[letter_idx as usize as usize] -= 1;
                s.push(letter_idx + b'a');
                repeat_cnt += 1;
            }
        }
        unsafe { std::string::String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, repeat_limit: i32, expected: &str) {
        assert!(s.len() >= 1);
        assert!(repeat_limit >= 1);
        assert!(repeat_limit as usize <= s.len());
        assert!(s.len() <= 100_000);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(
            Solution::repeat_limited_string(s.to_owned(), repeat_limit),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("cczazcc", 3, "zzcccac")
    }

    #[test]
    fn ex2() {
        test("aababab", 2, "bbabaa")
    }
}
