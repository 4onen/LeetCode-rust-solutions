// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/

pub struct Solution;

// Initial sol'n (but .into_bytes() instead of .bytes())
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut balance = 0;
        let mut worst_balance = 0;
        for b in s.into_bytes() {
            match b {
                b'[' => balance -= 1,
                b']' => balance += 1,
                _ => panic!("Invalid character: {}", b as char),
            }
            worst_balance = worst_balance.max(balance);
        }
        (worst_balance + 1) / 2
    }
}

// Iterator-only sol'n (but .into_bytes() instead of .bytes())
// impl Solution {
//     pub fn min_swaps(s: String) -> i32 {
//         let worst_balance = s.into_bytes().into_iter().scan(0, |balance, b| {
//             match b {
//                 b'[' => *balance -= 1,
//                 b']' => *balance += 1,
//                 _ => unreachable!("Invalid character: {}", b as char),
//             }
//             Some(*balance)
//         }).max().unwrap();
//         (worst_balance + 1) / 2
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 2);
        assert!(s.len() <= 1_000_000);
        assert!(s.len() % 2 == 0);
        let mut open = 0;
        for b in s.bytes() {
            match b {
                b'[' => open += 1,
                b']' => open -= 1,
                _ => panic!("Invalid character: {}", b as char),
            }
        }
        assert_eq!(open, 0, "Input string cannot be balanced");
        assert_eq!(Solution::min_swaps(s.to_string()), expected)
    }

    #[test]
    fn ex1() {
        test("][][", 1)
    }

    #[test]
    fn ex2() {
        test("]]][[[", 2)
    }

    #[test]
    fn ex3() {
        test("[]", 0)
    }
}
