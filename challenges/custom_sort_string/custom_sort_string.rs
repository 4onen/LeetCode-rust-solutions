// https://leetcode.com/problems/custom-sort-string/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn custom_sort_string(order: String, s: String) -> String {
//         let order = order
//             .bytes()
//             .enumerate()
//             .fold([0u8; 26], |mut order, (i, c)| {
//                 order[c as usize - 'a' as usize] = i as u8;
//                 order
//             });
//         let mut s = s.into_bytes();
//         s.sort_unstable_by_key(|&c| order[c as usize - 'a' as usize]);
//         String::from_utf8(s).unwrap()
//     }
// }

// Memory optimization maybe?
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut new_order = [0u8; 26];
        for (i, c) in order.bytes().enumerate() {
            new_order[c as usize - 'a' as usize] = i as u8;
        }
        let order = new_order;
        let mut s = s.into_bytes();
        s.sort_unstable_by_key(|&c| order[c as usize - 'a' as usize]);
        String::from_utf8(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_unordered_chars_present(s: &str, order: &str, result: &str) -> bool {
        s.chars().all(|c| {
            if order.contains(c) {
                result.contains(c)
            } else {
                true
            }
        })
    }

    fn ordered_chars_only(order: &str, result: &str) -> String {
        result.chars().filter(|&c| order.contains(c)).collect()
    }

    #[test]
    fn ex1() {
        let order = "cba";
        let s = "abcd";
        let expected_ordered = "cba";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn ex2() {
        let order = "bcafg";
        let s = "abcd";
        let expected_ordered = "bca";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn discussion_case1() {
        let order = "abcdefghijklmnopqrstuvwxyz";
        let s = "abcde";
        let expected_ordered = "abcde";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn discussion_case2() {
        let order = "xyz";
        let s = "xyzab";
        let expected_ordered = "xyz";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn discussion_case3() {
        let order = "pqrs";
        let s = "pqrstuvwx";
        let expected_ordered = "pqrs";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn discussion_case4() {
        let order = "aeiou";
        let s = "hello";
        let expected_ordered = "eo";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn discussion_case5() {
        let order = "cba";
        let s = "cba";
        let expected_ordered = "cba";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn discussion_case6() {
        let order = "abc";
        let s = "abcd";
        let expected_ordered = "abc";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }

    #[test]
    fn myex1() {
        let order = "cba";
        let s = "abccba";
        let expected_ordered = "ccbbaa";
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(check_unordered_chars_present(s, order, &result));
        assert_eq!(ordered_chars_only(order, &result), expected_ordered);
    }
}
