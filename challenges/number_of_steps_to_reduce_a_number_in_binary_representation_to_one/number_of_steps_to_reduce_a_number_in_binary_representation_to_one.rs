// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn num_steps(s: String) -> i32 {
//         let mut count = 0;
//         let mut last: u8 = b'0';
//         for b in s.bytes().skip(1).rev() {
//             match (b, last) {
//                 (b'0', b'1') => {
//                     count += 2;
//                     last = b'1';
//                 }
//                 (b'0', b'0') | (b'1', b'1') | (b'1', b'0') => {
//                     count += 1;
//                     last = b;
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         if last == b'1' {
//             count += 2;
//         }
//         return count;
//     }
// }

// Bool instead of multiple byte comparisons
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut count = 0;
        let mut tripped = false;
        for b in s.bytes().skip(1).rev() {
            if tripped {
                match b {
                    b'0' => count += 2,
                    b'1' => count += 1,
                    _ => unreachable!(),
                }
            } else {
                count += 1;
                tripped = b == b'1'
            }
        }
        if tripped {
            count += 2;
        }
        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, expected: i32) {
        assert!(input.len() > 0);
        assert!(input.len() <= 500);
        for c in input.bytes() {
            assert!(c == b'0' || c == b'1');
        }
        assert!(input.bytes().next().unwrap() == b'1');
        assert_eq!(Solution::num_steps(input.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("1101", 6)
    }

    #[test]
    fn ex2() {
        test("10", 1)
    }

    #[test]
    fn ex3() {
        test("1", 0)
    }

    #[test]
    fn myex3() {
        test("11", 3)
    }

    #[test]
    fn myex4() {
        test("100", 2)
    }

    #[test]
    fn myex5() {
        // 101
        // 110
        //  11
        // 100
        //  10
        //   1
        test("101", 5)
    }

    #[test]
    fn myex6() {
        test("110", 4)
    }

    #[test]
    fn myex7() {
        test("111", 4)
    }

    #[test]
    fn myex8() {
        test("1000", 3)
    }

    #[test]
    fn myex9() {
        // 1001
        // 1010
        //  101
        test("1001", 7)
    }

    #[test]
    fn myex10() {
        test("1010", 6)
    }

    #[test]
    fn myex11() {
        // 1011
        // 1100
        //  110
        //   11
        //  100
        //   10
        //    1
        test("1011", 6)
    }

    #[test]
    fn myex12() {
        test("1100", 5)
    }

    #[test]
    fn myex14() {
        test("1110", 5)
    }

    #[test]
    fn myex15() {
        test("1111", 5)
    }

    #[test]
    fn myex16() {
        test("10000", 4)
    }

    #[test]
    fn myex17() {
        // 10001
        // 10010
        //  1001
        test("10001", 9)
    }

    #[test]
    fn discussion_case1() {
        test(
            "1111011110000011100000110001011011110010111001010111110001",
            85,
        );
    }
}
