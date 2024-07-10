// https://leetcode.com/problems/crawler-log-folder/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_operations(logs: Vec<String>) -> i32 {
//         let mut depth: u16 = 0;
//         for log in logs {
//             match log.as_bytes()[..2] {
//                 [b'.', b'.'] => {
//                     depth = depth.saturating_sub(1);
//                 }
//                 [b'.', _] => {}
//                 _ => {
//                     depth += 1;
//                 }
//             }
//         }
//         depth as i32
//     }
// }

// String pattern matching
// impl Solution {
//     pub fn min_operations(logs: Vec<String>) -> i32 {
//         let mut depth: u16 = 0;
//         for log in logs {
//             match log.as_str() {
//                 "../" => {
//                     depth = depth.saturating_sub(1);
//                 }
//                 "./" => {}
//                 _ => {
//                     depth += 1;
//                 }
//             }
//         }
//         depth as i32
//     }
// }

// Byte array with bool conversion subtraction
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        assert!(logs.len() <= 1000);
        let mut depth: i16 = 0;
        for log in logs {
            match log.as_bytes()[..2] {
                [b'.', b'.'] => {
                    depth -= (depth > 0) as i16;
                }
                [b'.', _] => {}
                _ => {
                    depth += 1;
                }
            }
        }
        depth as i32
    }
}

// String pattern matching with bool conversion subtraction
// impl Solution {
//     pub fn min_operations(logs: Vec<String>) -> i32 {
//         assert!(logs.len() <= 1000);
//         let mut depth = 0;
//         for log in logs {
//             match log.as_str() {
//                 "../" => {
//                     depth -= (depth > 0) as i32;
//                 }
//                 "./" => {}
//                 _ => {
//                     depth += 1;
//                 }
//             }
//         }
//         depth
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(logs: &[&str], expected: i32) {
        let logs = logs.iter().map(|&s| s.to_string()).collect();
        assert_eq!(Solution::min_operations(logs), expected);
    }

    #[test]
    fn ex1() {
        test(&["d1/","d2/","../","d21/","./"], 2);
    }

    #[test]
    fn ex2() {
        test(&["d1/","d2/","./","d3/","../","d31/"], 3);
    }

    #[test]
    fn ex3() {
        test(&["d1/","../","../","../"], 0);
    }
}
