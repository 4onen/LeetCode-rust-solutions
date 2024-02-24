// https://leetcode.com/problems/maximum-score-after-splitting-a-string/

pub struct Solution;

// First idea: scan right to left counting 1s, then left to right counting 0s
// Time: O(n)
// Space: O(n)
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

// Tipped idea: Count total number of 1s, then scan left to right counting 0s
// Time: O(n)
// Space: O(1)
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

    #[test]
    fn ex1() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::max_score("00111".to_string()), 5);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::max_score("1111".to_string()), 3);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::max_score("000".to_string()), 2);
        assert_eq!(Solution::max_score("0000".to_string()), 3);
        assert_eq!(Solution::max_score("00000".to_string()), 4);
    }
}
