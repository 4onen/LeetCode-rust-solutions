// https://leetcode.com/problems/maximum-score-after-splitting-a-string/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn max_score(s: String) -> i32 {
//         let ns: Vec<bool> = s.as_bytes().into_iter().map(|&b| b == b'1').collect();
//         let ones_right = ns
//             .iter()
//             .rev()
//             .scan(0, |acc, &b| {
//                 *acc += b as i32;
//                 Some(*acc)
//             })
//             .collect::<Vec<_>>();
//         let zeroes_left = ns.into_iter().scan(0, |acc, b| {
//             *acc += !b as i32;
//             Some(*acc)
//         });
//         Iterator::zip(zeroes_left, ones_right.into_iter().rev().skip(1))
//             .map(|(zeroes, ones)| zeroes + ones)
//             .max()
//             .unwrap() as i32
//     }
// }

// Constant space after initial filter. Oop.
// impl Solution {
//     pub fn max_score(s: String) -> i32 {
//         let ns: Vec<bool> = s.as_bytes().into_iter().map(|&b| b == b'1').collect();
//         let total_ones = ns.iter().filter(|&&b| b).count() as i32;
//         ns[..ns.len() - 1]
//             .into_iter()
//             .scan((0, total_ones), |acc, b| {
//                 if *b {
//                     acc.1 -= 1;
//                 } else {
//                     acc.0 += 1;
//                 }
//                 Some(acc.0 + acc.1)
//             })
//             .max()
//             .unwrap_or(0)
//     }
// }

// Well now I know the LeetCode memory number is B.S.
// True constant space.
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let ns = s.as_bytes().into_iter().map(|&b| b == b'1');
        let total_ones = ns.clone().filter(|&b| b).count() as i32;
        let mut ns = ns;
        ns.next_back(); // Discard last value to make this scan only nums[0..n-1]
        ns.scan((0, total_ones), |acc, b| {
            if b {
                acc.1 -= 1;
            } else {
                acc.0 += 1;
            }
            Some(acc.0 + acc.1)
        })
        .max()
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 2);
        assert!(s.len() <= 500);
        for &b in s.as_bytes() {
            assert!(b == b'1' || b == b'0');
        }
        assert_eq!(Solution::max_score(s.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("011101", 5)
    }

    #[test]
    fn ex2() {
        test("00111", 5)
    }

    #[test]
    fn ex3() {
        test("1111", 3)
    }

    #[test]
    fn discussion_case1() {
        test("00", 1)
    }
}
