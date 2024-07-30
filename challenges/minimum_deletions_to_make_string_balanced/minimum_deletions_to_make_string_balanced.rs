// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/

pub struct Solution;

// O(n^2) solution (broken)
// impl Solution {
//     pub fn minimum_deletions(s: String) -> i32 {
//         let bytes = s.as_bytes();
//         assert!(bytes.len() >= 1);
//         assert!(bytes.len() <= 100_000);
//         let mut min_deletions = i32::MAX;
//         for i in 1..bytes.len() {
//             if bytes[i] != bytes[i - 1] {
//                 let left_b = bytes[..i].iter().filter(|&&b| b == b'b').count() as i32;
//                 let right_a = bytes[i + 1..].iter().filter(|&&b| b == b'a').count() as i32;
//                 min_deletions = std::cmp::min(min_deletions, left_b + right_a);
//             }
//         }
//         if min_deletions == i32::MAX {
//             0
//         } else {
//             min_deletions
//         }
//     }
// }

// prefix sum solution (broken)
// impl Solution {
//     pub fn minimum_deletions(s: String) -> i32 {
//         let bytes = s.as_bytes();
//         assert!(bytes.len() >= 1);
//         assert!(bytes.len() <= 100_000);
//         let total_a = bytes.iter().filter(|&&b| b == b'a').count() as i32;
//         let total_b = bytes.len() as i32 - total_a;
//         let mut a = 0;
//         let mut b = total_b;
//         let mut min_deletions = total_a;
//         for &byte in bytes {
//             match byte {
//                 b'a' => a += 1,
//                 b'b' => b -= 1,
//                 _ => unreachable!(),
//             }
//             min_deletions = min_deletions.min(total_a - a + b);
//         }
//         min_deletions.min(total_b)
//     }
// }

// impl Solution {
//     pub fn minimum_deletions(s: String) -> i32 {
//         let bytes = s.as_bytes();
//         assert!(bytes.len() >= 1);
//         assert!(bytes.len() <= 100_000);
//         let (total_a, first_b, last_a) = {
//             let mut total_a = 0;
//             let mut first_b = bytes.len() as i32-1;
//             let mut last_a = 0;
//             for (i, &byte) in bytes.into_iter().enumerate() {
//                 match byte {
//                     b'a' => {
//                         total_a += 1;
//                         last_a = i as i32;
//                     }
//                     b'b' if (i as i32) < first_b => {
//                         first_b = i as i32;
//                     }
//                     b'b' => {}
//                     _ => unreachable!(),
//                 }
//             }
//             (total_a, first_b, last_a)
//         };
//         if first_b > last_a {
//             return 0;
//         }
//         let total_b = bytes.len() as i32 - total_a;
//         // Everything before first_b is an 'a'
//         // Everything after last_a is a 'b'
//         // We need the number of each in the range [first_b, last_a]
//         let deletable_a = total_a - first_b;
//         let deletable_b = total_b - (bytes.len() as i32 - last_a - 1);
//         dbg!(total_a, total_b, first_b, last_a, deletable_a, deletable_b);
//         std::cmp::min(deletable_a, deletable_b) as i32
//     }
// }

// "Count b before last a" solution someone suggested (broken, obviously.)
// impl Solution {
//     pub fn minimum_deletions(s: String) -> i32 {
//         let bytes = s.as_bytes();
//         assert!(bytes.len() >= 1);
//         assert!(bytes.len() <= 100_000);
//         let mut total_b = 0;
//         let mut b_before_a = 0;
//         let mut first_b = bytes.len() as i32-1;
//         for (i,&byte) in bytes.into_iter().enumerate() {
//             if byte == b'b' {
//                 total_b += 1;
//                 if (i as i32) < first_b {
//                     first_b = i as i32;
//                 }
//             } else {
//                 b_before_a = total_b;
//             }
//         }
//         return b_before_a
//     }
// }

// Stack sol'n (Finally works)
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let bytes = s.as_bytes();
        assert!(bytes.len() >= 1);
        assert!(bytes.len() <= 100_000);
        let mut a_count = 0 as i32;
        let mut b_count = 0 as i32;
        for &byte in bytes {
            match byte {
                b'a' => {
                    if b_count > 0 {
                        b_count -= 1;
                    }
                    a_count += 1;
                }
                b'b' => {
                    b_count += 1;
                }
                _ => unreachable!(),
            }
        }
        (bytes.len() as i32) - (a_count as i32) - (b_count as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100_000);
        assert_eq!(Solution::minimum_deletions(s.to_string()), expected)
    }

    #[test]
    fn ex1() {
        test("aababbab", 2)
    }

    #[test]
    fn ex2() {
        test("bbaaaaabb", 2)
    }

    #[test]
    fn myex1() {
        test("a", 0)
    }

    #[test]
    fn myex2() {
        test("b", 0)
    }

    #[test]
    fn myex3() {
        test("ab", 0)
    }

    #[test]
    fn myex4() {
        test("ba", 1)
    }

    #[test]
    fn myex4_2() {
        test("bbaa", 2)
    }

    #[test]
    fn myex4_3() {
        test("bbabaa", 3)
    }

    #[test]
    fn myex4_4() {
        test("bbbaaaa", 3)
    }

    #[test]
    fn myex5() {
        test("aa", 0)
    }

    #[test]
    fn myex6() {
        test("bb", 0)
    }

    #[test]
    fn myex7() {
        test("abab", 1)
    }

    #[test]
    fn myex8() {
        test("ababab", 2)
    }

    #[test]
    fn myex9() {
        test("abababab", 3)
    }

    #[test]
    fn discussion_case1() {
        test("abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab",1165)
    }

    #[test]
    fn discussion_case2() {
        test("abbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbbbaaaaabbbbb", 7665)
    }

    #[test]
    fn discussion_case3() {
        test("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 4000);
    }

    #[test]
    fn my_extreme_ex1() {
        const HALF_MAX_LEN: usize = 50000;
        let input_as = "a".repeat(HALF_MAX_LEN);
        let input_bs = "b".repeat(HALF_MAX_LEN);
        test(&[input_as.as_str(), input_bs.as_str()].join(""), 0);
        test(&[input_bs.as_str(), input_bs.as_str()].join(""), 0);
        test(
            &[input_bs.as_str(), input_as.as_str()].join(""),
            HALF_MAX_LEN as i32,
        );
        test(&[input_as.as_str(), input_as.as_str()].join(""), 0);
    }

    #[test]
    fn my_extreme_ex2() {
        let input_as = "a".repeat(70000);
        let input_bs = "b".repeat(30000);
        test(&[input_as.as_str(), input_bs.as_str()].join(""), 0);
        test(&[input_bs.as_str(), input_bs.as_str()].join(""), 0);
        test(
            &[input_bs.as_str(), input_as.as_str()].join(""),
            input_bs.len() as i32,
        );
    }

    #[test]
    fn my_extreme_ex3() {
        let input_as = "a".repeat(30000);
        let input_bs = "b".repeat(70000);
        test(&[input_as.as_str(), input_bs.as_str()].join(""), 0);
        test(
            &[input_bs.as_str(), input_as.as_str()].join(""),
            input_as.len() as i32,
        );
        test(&[input_as.as_str(), input_as.as_str()].join(""), 0);
    }

    #[test]
    fn failing_case1() {
        test("ababaaaabbbbbaaababbbbbbaaabbaababbabbbbaabbbbaabbabbabaabbbababaa", 25);
    }
}
