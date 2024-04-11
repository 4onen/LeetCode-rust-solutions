// https://leetcode.com/problems/remove-k-digits/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn remove_kdigits(num: String, k: i32) -> String {
//         let num = num.as_bytes();
//         assert!(k >= 1);
//         let mut k = k as u32;
//         assert!(num.len() <= 100_000);
//         assert!(num.len() as u32 >= k);
//         let mut monotone_stack = Vec::with_capacity(num.len());
//         let mut i = 0;
//         loop {
//             if i == num.len() {
//                 break;
//             }
//             let digit = num[i];
//             while k > 0 && !monotone_stack.is_empty() && *monotone_stack.last().unwrap() > digit {
//                 monotone_stack.pop();
//                 k -= 1;
//             }
//             match digit {
//                 b'0' => {
//                     if !monotone_stack.is_empty() {
//                         monotone_stack.push(digit);
//                     }
//                 }
//                 _ => {
//                     monotone_stack.push(digit);
//                 }
//             }
//             i += 1;
//         }
//         if k >= monotone_stack.len() as u32 {
//             return "0".to_string();
//         } else {
//             let len = monotone_stack.len();
//             let new_len = len - k as usize;
//             // If k is still greater than 0, remove the last k digits
//             monotone_stack.truncate(new_len);
//             unsafe { std::string::String::from_utf8_unchecked(monotone_stack) }
//         }
//     }
// }

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        assert!(k >= 1);
        let mut k = k as u32;
        let mut monotone_stack = Vec::with_capacity(num.len());
        for digit in num.bytes() {
            while k > 0 && !monotone_stack.is_empty() && *monotone_stack.last().unwrap() > digit {
                monotone_stack.pop();
                k -= 1;
            }
            monotone_stack.push(digit);
        }
        // If k is still greater than 0, remove the last k digits
        monotone_stack
            .truncate(monotone_stack.len() - std::cmp::min(k as usize, monotone_stack.len()));
        let result = unsafe { std::string::String::from_utf8_unchecked(monotone_stack) }
            .trim_start_matches('0')
            .to_string();
        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num: &str, k: i32, expected: &str) {
        assert_eq!(
            Solution::remove_kdigits(num.to_string(), k),
            expected.to_string()
        );
    }

    #[test]
    fn ex1() {
        test("1432219", 3, "1219");
    }

    #[test]
    fn ex2() {
        test("10200", 1, "200");
    }

    #[test]
    fn ex3() {
        test("10", 2, "0");
    }

    #[test]
    fn discussion_case1() {
        test("1234567890", 9, "0");
    }

    #[test]
    fn discussion_case2() {
        test("123454", 1, "12344");
    }

    #[test]
    fn myex1() {
        test("123454", 2, "1234");
    }

    #[test]
    fn myex2() {
        test("123454", 3, "123");
    }

    #[test]
    fn my_extreme_ex1() {
        let input = "12354".repeat(20000);
        let expected = "0";
        test(&input, 5 * 20000, &expected);
    }
    #[test]
    fn my_extreme_ex2() {
        let input = "12354".repeat(20000);
        let expected = "111";
        test(&input, 5 * 20000 - 3, &expected);
    }
}
