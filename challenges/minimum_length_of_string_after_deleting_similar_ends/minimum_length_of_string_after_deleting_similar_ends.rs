// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn minimum_length(s: String) -> i32 {
//         let s = s.into_bytes();
//         if s.len() < 2 {
//             return s.len() as i32;
//         }
//         let mut front = 0u32;
//         let mut back = s.len() as u32 - 1;
//         loop {
//             if s[front as usize] != s[back as usize] {
//                 break (back - front + 1) as i32;
//             }
//             let mut f = front;
//             let mut b = back;
//             while f < b && s[f as usize] == s[front as usize] {
//                 f += 1;
//             }
//             while f < b && s[b as usize] == s[back as usize] {
//                 b -= 1;
//             }
//             if f == b && s[f as usize] == s[front as usize] {
//                 break 0;
//             } else if f == b {
//                 break 1;
//             }
//             front = f;
//             back = b;
//         }
//     }
// }

// More rusty sol'n
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut bytes = s.as_bytes();
        while bytes.len() > 1 && bytes[0] == bytes[bytes.len() - 1] {
            let first = bytes[0];
            loop {
                bytes = &bytes[1..];
                if bytes.len() <= 0 || bytes[0] != first {
                    break;
                }
            }
            while bytes.len() > 0 && bytes[bytes.len() - 1] == first {
                bytes = &bytes[..bytes.len() - 1];
            }
        }
        bytes.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::minimum_length("ca".to_string()), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::minimum_length("a".to_string()), 1);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(Solution::minimum_length("aaaaaaaaaaaaa".to_string()), 0);
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::minimum_length(
                "bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb".to_string()
            ),
            1
        );
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(
            Solution::minimum_length(
                "aaaaaabbbbbababababababbabaabababbccccccbbbbbbaaa".to_string()
            ),
            29
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::minimum_length("cbc".to_string()), 1);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::minimum_length("cc".to_string()), 0);
    }
}
