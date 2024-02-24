// https://leetcode.com/problems/decode-ways/

pub struct Solution;

// 1D Array sol'n
// impl Solution {
//     pub fn num_decodings(s: String) -> i32 {
//         let bytes = s.as_bytes();
//         let mut arr = vec![0; s.len() + 1];
//         arr[0] = 1;
//         arr[1] = if bytes[0] != b'0' { 1 } else { 0 };

//         for i in 2..=s.len() {
//             let one_step = bytes[i - 1] - b'0';
//             let two_step = unsafe { std::str::from_utf8_unchecked(&bytes[i - 2..i]) }
//                 .parse::<u8>()
//                 .unwrap();

//             if one_step >= 1 {
//                 arr[i] += arr[i - 1];
//             }

//             if two_step >= 10 && two_step <= 26 {
//                 arr[i] += arr[i - 2];
//             }
//         }

//         arr[bytes.len()]
//     }
// }

// Lookbehind state variables sol'n
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut prev = 1;
        let mut curr = if bytes[0] != b'0' { 1 } else { 0 };

        for i in 2..=s.len() {
            let one_step = bytes[i - 1] - b'0';
            let two_step = (bytes[i - 2] - b'0') * 10 + (bytes[i - 1] - b'0');

            let mut next = 0;

            if one_step >= 1 {
                next += curr;
            }

            if two_step >= 10 && two_step <= 26 {
                next += prev;
            }

            prev = curr;
            curr = next;
        }

        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::num_decodings("101".to_string()), 1);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::num_decodings("1010".to_string()), 1);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::num_decodings("11111".to_string()), 8);
    }
}
