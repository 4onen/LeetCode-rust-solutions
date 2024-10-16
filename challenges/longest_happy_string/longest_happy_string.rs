// https://leetcode.com/problems/longest-happy-string/

pub struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        const fn downrange(i: i32) -> u8 {
            assert!(i >= 0);
            assert!(i <= 100);
            i as u8
        }
        let a = downrange(a);
        let b = downrange(b);
        let c = downrange(c);
        let mut res = Vec::with_capacity(a as usize + b as usize + c as usize);
        let mut count = [(a, b'a'), (b, b'b'), (c, b'c')];
        count.sort_unstable();
        let mut last_count = 1;
        while count[2].0 > 0 {
            let (max_count, max_char) = count[2];
            let (mid_count, mid_char) = count[1];
            if max_count == 0 {
                break;
            }
            let last = res.last().copied().unwrap_or(b' ');
            match last_count {
                2 if last == max_char && mid_count > 0 => {
                    res.push(mid_char);
                    count[1].0 -= 1;
                    last_count = 1;
                }
                2 if last == max_char => {
                    break; // No more mid_char, we can't add max_char
                }
                _ => {
                    res.push(max_char);
                    count[2].0 -= 1;
                    last_count = 1 + (last == max_char) as u8;
                }
            }
            count.sort_unstable();
        }
        unsafe { std::string::String::from_utf8_unchecked(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(a: u8, b: u8, c: u8, expected: &[&str]) {
        assert!(a + b + c > 0);
        for e in expected {
            assert!(e.len() > 0);
            assert!(e.len() <= (a as usize + b as usize + c as usize));
            property_test_str(a, b, c, e);
        }
        let result = property_test(a, b, c);
        // Check if the result is in the expected array
        for e in expected {
            if result == *e {
                return;
            }
        }
        assert!(
            false,
            "Result '{}' not in expected array: {:?}",
            result, expected
        );
    }

    const fn property_test_str(a: u8, b: u8, c: u8, result: &str) {
        let result_str = result;
        let result = result_str.as_bytes();
        assert!(result.len() > 0);
        let mut counts = [0u8; 3];
        let mut last = b' ';
        let mut cnt = 0;
        let mut i = 0;
        while i < result.len() {
            let ch = result[i];
            assert!(ch == b'a' || ch == b'b' || ch == b'c');
            counts[(ch - b'a') as usize] += 1;
            if ch == last {
                cnt += 1;
                assert!(cnt < 2, "{}", result_str);
            } else {
                cnt = 0;
                last = ch;
            }
            i += 1;
        }
        assert!(counts[0] <= a); //, "Too many 'a' (Found {} / {})", counts[0], a);
        assert!(counts[1] <= b); //, "Too many 'b' (Found {} / {})", counts[1], b);
        assert!(counts[2] <= c); //, "Too many 'c' (Found {} / {})", counts[2], c);
    }

    fn property_test(a: u8, b: u8, c: u8) -> String {
        let result = Solution::longest_diverse_string(a as i32, b as i32, c as i32);
        property_test_str(a, b, c, &result);
        result
    }

    #[test]
    fn ex1() {
        test(1, 1, 7, &["ccbccacc", "ccbccacc"])
    }

    #[test]
    fn ex2() {
        test(7, 1, 0, &["aabaa"])
    }

    #[test]
    fn discussion_case0() {
        test(0, 0, 7, &["cc"])
    }

    #[test]
    fn discussion_case0_1() {
        test(0, 7, 0, &["bb"])
    }

    #[test]
    fn discussion_case0_2() {
        test(7, 0, 0, &["aa"])
    }

    #[test]
    fn discussion_case1() {
        property_test(1, 4, 5);
    }

    #[test]
    fn discussion_case2() {
        test(
            9,
            3,
            1,
            &[
                "aabaabaabaaca",
                "aabaabaacaaba",
                "aabaacaabaaba",
                "aacaabaabaaba",
            ],
        );
    }

    #[test]
    fn discussion_case3() {
        property_test(0, 8, 11);
    }

    #[test]
    fn discussion_case100() {
        property_test(100, 100, 100);
    }

    #[test]
    fn discussion_case4() {
        property_test(58, 50, 87);
    }

    #[test]
    fn discussion_case5() {
        property_test(3, 40, 7);
    }

    #[test]
    fn discussion_case6() {
        property_test(67, 53, 99);
    }
}
