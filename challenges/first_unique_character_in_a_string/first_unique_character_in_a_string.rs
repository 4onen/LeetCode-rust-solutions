// https://leetcode.com/problems/first-unique-character-in-a-string/

pub struct Solution;

// Two bool arr sol'n
// impl Solution {
//     pub fn first_uniq_char(s: String) -> i32 {
//         let mut seen = vec![false; 26];
//         let mut repeated = vec![false; 26];
//         let bytes = s.into_bytes();
//         for &b in bytes.iter() {
//             let index = (b - b'a') as usize;
//             if seen[index] {
//                 repeated[index] = true;
//             } else {
//                 seen[index] = true;
//             }
//         }
//         for (i, b) in bytes.iter().enumerate() {
//             let index = (b - b'a') as usize;
//             if !repeated[index] {
//                 return i as i32;
//             }
//         }
//         -1
//     }
// }

// One u8 arr sol'n
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut seen = vec![0u8; 26];
        for b in s.bytes() {
            seen[(b - b'a') as usize] = seen[(b - b'a') as usize].saturating_add(1);
        }
        for (i, b) in s.bytes().enumerate() {
            if seen[(b - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

// Iterator-heavy one u8 arr sol'n
// impl Solution {
//     pub fn first_uniq_char(s: String) -> i32 {
//         let mut seen = vec![0u8; 26];
//         for b in s.bytes() {
//             seen[(b - b'a') as usize] = seen[(b - b'a') as usize].saturating_add(1);
//         }
//         s.bytes()
//             .enumerate()
//             .find(|(_, b)| seen[(*b - b'a') as usize] == 1)
//             .map(|(i, _)| i as i32)
//             .unwrap_or(-1)
//     }
// }

// Excessive abstraction sol'n
// impl Solution {
//     pub fn first_uniq_char(s: String) -> i32 {
//         #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//         enum Appearances {
//             Unseen,
//             Seen,
//             Repeated,
//         }
//         impl Appearances {
//             const fn new() -> Self {
//                 Appearances::Unseen
//             }
//             fn increment(&mut self) {
//                 *self = match self {
//                     Appearances::Unseen => Appearances::Seen,
//                     Appearances::Seen => Appearances::Repeated,
//                     Appearances::Repeated => Appearances::Repeated,
//                 }
//             }
//         }
//         let mut seen = vec![Appearances::new(); 26];
//         for b in s.bytes() {
//             seen[(b - b'a') as usize].increment();
//         }
//         for (i, b) in s.bytes().enumerate() {
//             if seen[(b - b'a') as usize] == Appearances::Seen {
//                 return i as i32;
//             }
//         }
//         -1
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}
