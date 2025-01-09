// https://leetcode.com/problems/reformat-the-string/

pub struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut sb = s.into_bytes();
        let len = sb.len() as u16;
        let mut content = [0u16; (b'z' - b'0' + 1) as usize];
        for b in sb.drain(..) {
            content[(b - b'0') as usize] += 1;
        }
        let numbers = content[..=(b'9' - b'0') as usize].iter().sum::<u16>();
        let letters = len - numbers;
        if u16::abs_diff(letters, numbers) >= 2 {
            String::new()
        } else {
            let mut placing_number = numbers > letters;
            let mut number_head = 0u8;
            let mut letter_head = b'z' - b'a';
            loop {
                if placing_number {
                    while number_head <= (b'9' - b'0') && content[number_head as usize] == 0 {
                        number_head += 1;
                    }
                    if number_head > (b'9' - b'0') {
                        break;
                    }
                    sb.push(number_head + b'0');
                    content[number_head as usize] -= 1;
                } else {
                    while letter_head <= (b'z' - b'0') && content[letter_head as usize] == 0 {
                        letter_head += 1;
                    }
                    if letter_head > (b'z' - b'0') {
                        break;
                    }
                    sb.push(letter_head + b'0');
                    content[letter_head as usize] -= 1;
                }
                placing_number = !placing_number;
            }
            unsafe { std::string::String::from_utf8_unchecked(sb) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_content(s: &str) -> [u16; (b'z' - b'0' + 1) as usize] {
        let mut res = [0u16; (b'z' - b'0' + 1) as usize];
        for &b in s.as_bytes() {
            res[(b - b'0') as usize] += 1;
        }
        res
    }

    fn test(s: &str) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 500);
        let mut numbers = 0u16;
        let mut letters = 0u16;
        for &b in s.as_bytes() {
            assert!(b >= b'0');
            assert!(b <= b'9' || b >= b'a');
            assert!(b <= b'z');
            if b.is_ascii_digit() {
                numbers += 1;
            } else {
                letters += 1;
            }
        }
        let solvable = u16::abs_diff(numbers, letters) < 2;
        let result = Solution::reformat(s.to_owned());
        if solvable {
            dbg!(s,&result);
            assert_eq!(result.len(), s.len(), "Responded {:?}", result);
            let mut result_numbers = 0;
            for &b in s.as_bytes() {
                assert!(b >= b'0');
                assert!(b <= b'9' || b >= b'a');
                assert!(b <= b'z');
                if b.is_ascii_digit() {
                    result_numbers += 1;
                }
            }
            assert_eq!(result_numbers, numbers);
            let original_content = count_content(s);
            let result_content = count_content(&result);
            assert_eq!(result_content, original_content);
            // Actually check the condition, d'oh.
            let result_bytes = result.as_bytes();
            let mut number_last =! result_bytes[0].is_ascii_digit();
            for b in result_bytes {
                let is_number = b.is_ascii_digit();
                assert_ne!(is_number, number_last, "Responded {:?}", &result);
                number_last = is_number;
            }
        } else {
            assert_eq!(result, "");
        }
    }

    #[test]
    fn ex1() {
        test("a0b1c2")
    }

    #[test]
    fn ex2() {
        test("leetcode")
    }

    #[test]
    fn ex3() {
        test("1229857369")
    }

    #[test]
    fn myex1() {
        test("abcdefghijklmnopqrstuvwxyz01234567890123456789012345")
    }

    #[test]
    fn myex1_1() {
        test("abcdefghijklmnopqrstuvwxyz0123456789012345678901234")
    }

    #[test]
    fn myex1_2() {
        test("abcdefghijklmnopqrstuvwxyz012345678901234567890123")
    }

    #[test]
    fn myex1_3() {
        test("abcdefghijklmnopqrstuvwxyz012345678901234567890123456")
    }

    #[test]
    fn myex1_4() {
        test("abcdefghijklmnopqrstuvwxyz0123456789012345678901234567")
    }

    #[test]
    fn myex1_5() {
        test("abcdefghijklmnopqrstuvwxyz01234567890123456789012345678")
    }
}
