// https://leetcode.com/problems/count-the-number-of-consistent-strings/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
//         let allowed = {
//             let mut arr = [false; 26];
//             for b in allowed.bytes() {
//                 arr[(b - b'a') as usize] = true;
//             }
//             arr
//         };
//         words
//             .into_iter()
//             .filter(|word| word.bytes().all(|b| allowed[(b - b'a') as usize]))
//             .count() as i32
//     }
// }

// Bitmask sol'n
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed = {
            let mut arr = 0u32;
            for b in allowed.bytes() {
                arr |= 1 << (b - b'a');
            }
            arr
        };
        words
            .into_iter()
            .filter(|word| word.bytes().all(|b| allowed & (1 << (b - b'a')) != 0))
            .count() as i32
    }
}

// Raw loop sol'n
// impl Solution {
//     pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
//         let allowed = {
//             let mut arr = 0u32;
//             for b in allowed.bytes() {
//                 arr |= 1 << (b - b'a');
//             }
//             arr
//         };
//         let mut count = 0;
//         'outer: for word in words {
//             for b in word.bytes() {
//                 if allowed & (1 << (b - b'a')) == 0 {
//                     continue 'outer;
//                 }
//             }
//             count += 1;
//         }
//         count
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(allowed: &str, words: &[&str], expected: i32) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 10_000);
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 10);
            for b in word.bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert!(allowed.len() >= 1);
        assert!(allowed.len() <= 26);
        for (i, b) in allowed.bytes().enumerate() {
            assert!(b >= b'a');
            assert!(b <= b'z');
            for (j, bb) in allowed.bytes().enumerate() {
                if i != j {
                    assert_ne!(b, bb);
                }
            }
        }
        assert_eq!(
            Solution::count_consistent_strings(
                allowed.to_string(),
                words.iter().map(|&s| s.to_string()).collect()
            ),
            expected
        )
    }

    #[test]
    fn ex1() {
        test("ab", &["ad", "bd", "aaab", "baa", "badab"], 2)
    }

    #[test]
    fn ex2() {
        test("abc", &["a", "b", "c", "ab", "ac", "bc", "abc"], 7)
    }

    #[test]
    fn ex3() {
        test("cad", &["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"], 4)
    }
}
