// https://leetcode.com/problems/shortest-palindrome/

pub struct Solution;

// O(n^2) solution generating each palindrome.
// too slow on large un-self-matchable inputs.
// impl Solution {
//     pub fn shortest_palindrome(s: String) -> String {
//         let s = s.as_bytes();
//         let n = s.len();
//         assert!(n <= 50_000);
//         if n == 0 {
//             return "".to_string();
//         }
//         for i in 0..n {
//             let iterated = std::iter::Iterator::chain(s.iter().skip(n - i).rev(), s.iter());
//             if std::iter::Iterator::eq(iterated.clone(), iterated.clone().rev()) {
//                 return unsafe {
//                     std::string::String::from_utf8_unchecked(iterated.copied().collect())
//                 };
//             }
//         }
//         return "".to_string();
//     }
// }

// O(n^2) solution finding longest palindromic prefix naively.
// Much improved (can actually complete my test suite) but still too slow.
// Moving is_palindrome inside shortest_palindrome doubles the speed, likely
// due to loop inlining.
// impl Solution {
//     pub fn shortest_palindrome(s: String) -> String {
//         const fn is_palindrome(s: &[u8]) -> bool {
//             let n = s.len();
//             let mut i = 0;
//             while i < n/2 {
//                 if s[i as usize] != s[n as usize- i as usize- 1] {
//                     return false;
//                 }
//                 i+=1;
//             }
//             true
//         }
//         let s = s.as_bytes();
//         let n = s.len();
//         assert!(n <= 50_000);
//         if n == 0 {
//             return "".to_string();
//         }
//         let mut longest_palindromic_prefix = 0;
//         for i in 0..n {
//             if is_palindrome(&s[..=i]) {
//                 longest_palindromic_prefix = i + 1;
//             }
//         }
//         unsafe {
//             std::string::String::from_utf8_unchecked(
//                 s.iter().copied()
//                     .skip(longest_palindromic_prefix)
//                     .rev()
//                     .chain(s.into_iter().copied())
//                     .collect(),
//             )
//         }
//     }
// }

// Above, but with buffer reuse rather than chained iterators
// Turns out to be slower, likely due to the iterator chain being optimized
// impl Solution {
//     pub fn shortest_palindrome(s: String) -> String {
//         const fn is_palindrome(s: &[u8]) -> bool {
//             let n = s.len();
//             let mut i = 0;
//             while i < n/2 {
//                 if s[i as usize] != s[n as usize- i as usize- 1] {
//                     return false;
//                 }
//                 i+=1;
//             }
//             true
//         }
//         let mut s = s.into_bytes();
//         let n = s.len();
//         assert!(n <= 50_000);
//         if n == 0 {
//             return "".to_string();
//         }
//         let mut longest_palindromic_prefix = 1;
//         for i in 1..n {
//             if is_palindrome(&s[..=i]) {
//                 longest_palindromic_prefix = i + 1;
//             }
//         }
//         // Characters to copy to the front reversed
//         let chars_to_copy_rev = n - longest_palindromic_prefix;
//         s.reserve_exact(chars_to_copy_rev);
//         unsafe { s.set_len(n + chars_to_copy_rev) };
//         s.copy_within(..n, chars_to_copy_rev);
//         let chars_before_end = s.len() - chars_to_copy_rev;
//         s.copy_within(chars_before_end.., 0);
//         s[..chars_to_copy_rev].reverse();
//         unsafe { std::string::String::from_utf8_unchecked(s) }
//     }
// }

// O(n) solution based on KMP
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let s = s.into_bytes();
        let n = s.len();
        assert!(n <= 50_000);
        if n == 0 {
            return "".to_string();
        }
        let mut analysis_buf = std::vec::Vec::with_capacity(2 * n + 1);
        analysis_buf.extend(s.iter().copied());
        analysis_buf.push(0);
        analysis_buf.extend(s.iter().copied().rev());
        let mut longest_prefix_that_is_suffix = vec![0u32; analysis_buf.len()];
        for i in 1..analysis_buf.len() {
            let mut len_longest_so_far = longest_prefix_that_is_suffix[i - 1];
            while (len_longest_so_far > 0) && (analysis_buf[i] != analysis_buf[len_longest_so_far as usize])
            {
                len_longest_so_far = longest_prefix_that_is_suffix[len_longest_so_far  as usize- 1];
            }
            len_longest_so_far += (analysis_buf[i] == analysis_buf[len_longest_so_far as usize]) as u32;
            longest_prefix_that_is_suffix[i] = len_longest_so_far;
        }
        let longest_palindromic_prefix = *longest_prefix_that_is_suffix.last().unwrap();
        // Iterator chain is faster than buffer reuse
        unsafe {
            std::string::String::from_utf8_unchecked(
                s.iter().copied()
                    .skip(longest_palindromic_prefix as usize)
                    .rev()
                    .chain(s.iter().copied())
                    .collect(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Scale ranges from 1 to 1000
    const BIG_TEST_SCALE: usize = 1000;

    fn test(s: &str, expected: &str) {
        assert_eq!(Solution::shortest_palindrome(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("aacecaaa", "aaacecaaa")
    }

    #[test]
    fn ex2() {
        test("abcd", "dcbabcd")
    }

    #[test]
    fn discussion_case0() {
        test("", "")
    }

    #[test]
    fn discussion_case42000() {
        const N: usize = 42 * BIG_TEST_SCALE;
        test(
            unsafe { std::str::from_utf8_unchecked(&[b'a'; N]) },
            unsafe { std::str::from_utf8_unchecked(&[b'a'; N]) },
        )
    }

    #[test]
    fn my_extreme_ex1() {
        const N: usize = 50 * BIG_TEST_SCALE;
        test(
            unsafe { std::str::from_utf8_unchecked(&[b'a'; N]) },
            unsafe { std::str::from_utf8_unchecked(&[b'a'; N]) },
        )
    }

    #[test]
    fn my_extreme_ex2() {
        const N: usize = 50 * BIG_TEST_SCALE;
        let mut input_vec = vec![b'a'; N];
        *input_vec.last_mut().unwrap() = b'b';
        let input = unsafe { std::str::from_utf8_unchecked(&input_vec) };
        let mut expected_vec = vec![b'a'; N + 1];
        *expected_vec.last_mut().unwrap() = b'b';
        *expected_vec.first_mut().unwrap() = b'b';
        let expected = unsafe { std::str::from_utf8_unchecked(&expected_vec) };
        test(input, expected)
    }

    #[test]
    fn discussion_case_alphabet() {
        test("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz", "zyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz")
    }

    #[test]
    fn my_extreme_alphabet() {
        const N: usize = 50 * BIG_TEST_SCALE / 26;
        let input = "abcdefghijklmnopqrstuvwxyz".repeat(N);
        let expected = (("zyxwvutsrqponmlkjihgfedcba".repeat(N))
            + &("abcdefghijklmnopqrstuvwxyz".repeat(N)))
            .replacen("aa", "a", 1);
        test(&input, &expected)
    }
}
