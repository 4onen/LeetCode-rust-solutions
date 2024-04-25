// https://leetcode.com/problems/longest-ideal-subsequence/

pub struct Solution;

// Initial sol'n idea
// (Figured checking right would be too slow)
// (Let's try checking left instead)
// impl Solution {
//     pub fn longest_ideal_string(s: String, k: i32) -> i32 {
//         let s = s.as_bytes();
//         assert!(s.len() > 0);
//         assert!(s.len() <= 10usize.pow(5));
//         assert!(k >= 0);
//         assert!(k <= 25);
//         let k = k as u8;
//         let mut max_len_right = Vec::with_capacity(s.len());
//         let max_len_right_slots = max_len_right.spare_capacity_mut();
//         let mut max_idx = 0;
//         for i in (0..s.len() as u32).rev() {
//             let mut max_len_right_i = 0;
//             for j in i + 1..s.len() as u32 {
//                 if (s[i as usize] as i8 - s[j as usize] as i8).abs() as u8 <= k {
//                     max_len_right_i = std::cmp::max(max_len_right_i, unsafe {
//                         max_len_right_slots[j as usize].assume_init()
//                     });
//                 }
//                 if max_len_right_i > (s.len() as u32 - j + 1) {
//                     break;
//                 }
//             }
//             max_len_right_i += 1;
//             max_len_right_slots[i as usize].write(max_len_right_i);
//             if max_len_right_i > unsafe { max_len_right_slots[max_idx as usize].assume_init() } {
//                 max_idx = i;
//             }
//         }
//         (unsafe { max_len_right_slots[max_idx as usize].assume_init() }) as i32
//     }
// }

// Improved sol'n (Scan from left to right) (O(n^2) too slow)
// impl Solution {
//     pub fn longest_ideal_string(s: String, k: i32) -> i32 {
//         let s = s.as_bytes();
//         assert!(s.len() > 0);
//         assert!(s.len() <= 10usize.pow(5));
//         assert!(k >= 0);
//         assert!(k <= 25);
//         let k = k as u8;
//         let mut max_len_left = Vec::with_capacity(s.len());
//         let max_len_left_slots = max_len_left.spare_capacity_mut();
//         let mut max_idx = 0;
//         for i in 0..s.len() as u32 {
//             let mut max_len_left_i = 0;
//             for j in (0..i).rev() {
//                 if (s[i as usize] as i8 - s[j as usize] as i8).abs() as u8 <= k {
//                     max_len_left_i = std::cmp::max(max_len_left_i, unsafe {
//                         max_len_left_slots[j as usize].assume_init()
//                     });
//                 }
//                 if max_len_left_i > j {
//                     break;
//                 }
//             }
//             max_len_left_i += 1;
//             max_len_left_slots[i as usize].write(max_len_left_i);
//             if max_len_left_i > unsafe { max_len_left_slots[max_idx as usize].assume_init() } {
//                 max_idx = i;
//             }
//         }
//         unsafe { max_len_left.set_len(s.len()) };
//         max_len_left[max_idx as usize] as i32
//     }
// }

// Letter-space scan O(kn) where k is proportional to the size of alphabet
// we scan.
// impl Solution {
//     pub fn longest_ideal_string(s: String, k: i32) -> i32 {
//         assert!(s.len() > 0);
//         assert!(s.len() <= 10usize.pow(5));
//         assert!(k >= 0);
//         assert!(k <= 25);
//         let k = k as u8;
//         let mut alphabet_max_len = [0; 26];
//         for c in s.bytes() {
//             assert!((b'a'..=b'z').contains(&c));
//             let c = c - b'a';
//             let mut max_len = 0;
//             for i in 0..26u8 {
//                 if (c as i8 - i as i8).abs() as u8 <= k {
//                     max_len = std::cmp::max(max_len, alphabet_max_len[i as usize]);
//                 }
//             }
//             max_len += 1;
//             alphabet_max_len[c as usize] = max_len;
//         }
//         alphabet_max_len.into_iter().max().unwrap() as i32
//     }
// }

// Const-ifying the function because I can.
// impl Solution {
//     pub const fn longest_ideal_substr(s: &[u8], k: i32) -> i32 {
//         let k = k as i8;
//         let mut alphabet_max_len = [0; 26];
//         let mut i = 0;
//         while i < s.len() {
//             let c = s[i as usize] - b'a';
//             let mut max_len = 0;
//             let mut j = 0;
//             while j < alphabet_max_len.len() {
//                 if (c as i8 - j as i8).abs() <= k && alphabet_max_len[j] > max_len {
//                     max_len = alphabet_max_len[j];
//                 }
//                 j += 1;
//             }
//             alphabet_max_len[c as usize] = max_len + 1;
//             i += 1;
//         }
//         let mut j = 0;
//         let mut max_len = 0;
//         while j < alphabet_max_len.len() {
//             if alphabet_max_len[j] > max_len {
//                 max_len = alphabet_max_len[j];
//             }
//             j += 1;
//         }
//         max_len
//     }
//     pub fn longest_ideal_string(s: String, k: i32) -> i32 {
//         Self::longest_ideal_substr(s.as_bytes(), k)
//     }
// }

// Optimization on k that didn't help
// impl Solution {
//     pub const fn longest_ideal_substr(s: &[u8], k: i32) -> i32 {
//         match k {
//             25 => {
//                 return s.len() as i32;
//             }
//             0..=24 => {
//                 let k = k as i8;
//                 let mut alphabet_max_len = [0; 26];
//                 let mut i = 0;
//                 while i < s.len() {
//                     let c = s[i as usize] - b'a';
//                     let mut max_len = 0;
//                     let mut j = 0;
//                     while j < alphabet_max_len.len() {
//                         if (c as i8 - j as i8).abs() <= k && alphabet_max_len[j] > max_len {
//                             max_len = alphabet_max_len[j];
//                         }
//                         j += 1;
//                     }
//                     alphabet_max_len[c as usize] = max_len + 1;
//                     i += 1;
//                 }
//                 let mut j = 0;
//                 let mut max_len = 0;
//                 while j < alphabet_max_len.len() {
//                     if alphabet_max_len[j] > max_len {
//                         max_len = alphabet_max_len[j];
//                     }
//                     j += 1;
//                 }
//                 max_len
//             }
//             _ => {
//                 panic!("k must be between 0 and 25 inclusive");
//             }
//         }
//     }
//     pub fn longest_ideal_string(s: String, k: i32) -> i32 {
//         Self::longest_ideal_substr(s.as_bytes(), k)
//     }
// }

// Optimization on k that did help (Woo! Cut 3ms off the fastest time!)
impl Solution {
    pub const fn longest_ideal_substr(s: &[u8], k: i32) -> i32 {
        if k < 0 {
            return 1;
        }
        if k >= 25 {
            return s.len() as i32;
        }
        let k = k as i8;
        let mut alphabet_max_len = [0; 26];
        let mut i = 0;
        while i < s.len() {
            let c = s[i as usize] - b'a';
            let mut max_len = 0;
            let mut j = 0;
            while j < alphabet_max_len.len() {
                if (c as i8 - j as i8).abs() <= k && alphabet_max_len[j] > max_len {
                    max_len = alphabet_max_len[j];
                }
                j += 1;
            }
            alphabet_max_len[c as usize] = max_len + 1;
            i += 1;
        }
        let mut j = 0;
        let mut max_len = 0;
        while j < alphabet_max_len.len() {
            if alphabet_max_len[j] > max_len {
                max_len = alphabet_max_len[j];
            }
            j += 1;
        }
        max_len
    }
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        Self::longest_ideal_substr(s.as_bytes(), k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, k: i8, exp: u32) {
        let result = Solution::longest_ideal_string(s.to_string(), k as i32);
        assert!(result > 0);
        let result = result as u32;
        assert_eq!(result, exp);
    }

    #[test]
    fn ex1() {
        test("acfgbd", 2, 4)
    }

    #[test]
    fn ex2() {
        test("abcd", 3, 4)
    }

    #[test]
    fn discussion_case1() {
        test("eduktdb", 15, 5)
    }

    #[test]
    fn discussion_case2() {
        test("pvjcci", 1, 2)
    }

    #[test]
    fn myex1() {
        test("a", 1, 1)
    }

    #[test]
    fn myex2() {
        test("aa", 1, 2)
    }

    #[test]
    fn myex3() {
        test("aa", 2, 2)
    }

    #[test]
    fn myex4() {
        test("aaa", 2, 3)
    }

    #[test]
    fn myex5() {
        test("aaa", 1, 3)
    }

    #[test]
    fn myex6() {
        test("acca", 1, 2)
    }

    #[test]
    fn myex7() {
        test("accaa", 1, 3)
    }

    #[test]
    fn myex8() {
        test("accaa", 2, 5)
    }

    #[test]
    fn myex9() {
        test("aaccaccaa", 1, 5)
    }

    #[test]
    fn myex10() {
        test("aaccaccaa", 2, 9)
    }

    #[test]
    fn myex11() {
        test("aaccaddaa", 2, 7)
    }

    #[test]
    fn my_extreme_ex1() {
        let input = "a".repeat(10usize.pow(5));
        test(input.as_str(), 1, input.len() as u32)
    }

    #[test]
    fn failing_case1() {
        // Wrong assertion, due to mistyped assertion
        test("dll", 0, 2)
    }

    #[test]
    fn failing_case2() {
        // Wrong answer, due to assuming that if I'd eaten over half the
        // remaining string, there'd be no larger subseq. I can see now that
        // was a bad assumption.
        test(
            "hbgyjaaenqznzqniyuajcdcnfpqbuiyngqlzpxxbaxnwxixdumxbgnodaxrlntkqa",
            16,
            56,
        )
    }

    #[test]
    fn failing_case3() {
        // Time limit exceeded
        let input = include_str!("failing_case3.txt");
        test(input, 8, 48822)
    }
}
