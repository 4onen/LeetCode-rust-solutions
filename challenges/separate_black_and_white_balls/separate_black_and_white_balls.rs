// https://leetcode.com/problems/separate-black-and-white-balls/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn minimum_steps(s: String) -> i64 {
//         let mut s = s.into_bytes();
//         // First, find the leftmost black ball
//         let Some(mut leftmost_black) = s.iter().position(|&c| c == b'1') else {
//             return 0;
//         };
//         // Next, find the rightmost white ball
//         let Some(mut rightmost_white) = s.iter().rposition(|&c| c == b'0') else {
//             return 0;
//         };
//         // If the leftmost black ball is to the right of the rightmost white ball,
//         // we're done
//         let mut steps = 0;
//         while leftmost_black < rightmost_white {
//             // Swap the leftmost black ball with the rightmost white ball
//             s[leftmost_black] = b'0';
//             s[rightmost_white] = b'1';
//             // We can only swap adjacent balls, so count the number of steps
//             steps += // Wait this won't work here, because this would always be
//                      // more steps than necessary.
//             // Set the new leftmost black ball
//             leftmost_black = s.iter().skip(leftmost_black + 1).position(|&c| c == b'1').unwrap() + leftmost_black + 1;
//             // Set the new rightmost white ball
//             rightmost_white = s
//                 .iter()
//                 .take(rightmost_white)
//                 .rposition(|&c| c == b'0')
//                 .unwrap();
//         }
//         steps
//     }
// }

// Bubble sort sol'n (Too slow)
// impl Solution {
//     pub fn minimum_steps(s: String) -> i64 {
//         let mut s = s.into_bytes();
//         let mut moved = true;
//         let mut sorted_white = 0;
//         let mut steps = 0;
//         while moved {
//             moved = false;
//             for i in sorted_white..s.len() - 1 {
//                 if s[i] == b'1' && s[i + 1] == b'0' {
//                     s[i] = b'0';
//                     s[i + 1] = b'1';
//                     steps += 1;
//                     if !moved {
//                         sorted_white = std::cmp::max(1, i) - 1;
//                     }
//                     moved = true;
//                 }
//             }
//         }
//         steps
//     }
// }

// Optimized don't-actually-sort sol'n
// impl Solution {
//     pub fn minimum_steps(s: String) -> i64 {
//         let mut black_count = 0;
//         let mut step_count = 0;
//         for &c in s.as_bytes() {
//             match c {
//                 b'1' => black_count += 1,
//                 b'0' => step_count += black_count,
//                 _ => unreachable!(),
//             }
//         }
//         step_count
//     }
// }

// White-count sol'n (Helps remove data dependency between loop iters)
// impl Solution {
//     pub fn minimum_steps(s: String) -> i64 {
//         let mut white_count = 0;
//         let mut step_count = 0;
//         for (i, &c) in s.as_bytes().into_iter().enumerate() {
//             match c {
//                 b'1' => {}
//                 b'0' => {
//                     step_count += (i - white_count) as i64;
//                     white_count += 1;
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         step_count
//     }
// }

// Above but if statement to avoid unreachable!() check cost in debug
// impl Solution {
//     pub fn minimum_steps(s: String) -> i64 {
//         let mut white_count = 0;
//         let mut step_count = 0;
//         for (i, &c) in s.as_bytes().into_iter().enumerate() {
//             if c == b'0' {
//                 step_count += (i - white_count) as i64;
//                 white_count += 1;
//             }
//         }
//         step_count
//     }
// }

// Above but with a while loop over our byte view
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let s = s.as_bytes();
        let mut white_count = 0;
        let mut step_count = 0;
        let mut i = 0;
        while i < s.len() {
            if s[i] == b'0' {
                step_count += (i - white_count) as i64;
                white_count += 1;
            }
            i += 1;
        }
        step_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i64) {
        assert_eq!(Solution::minimum_steps(s.to_string()), expected)
    }

    #[test]
    fn ex1() {
        test("101", 1)
    }

    #[test]
    fn ex2() {
        test("100", 2)
    }

    #[test]
    fn ex3() {
        test("0111", 0)
    }

    #[test]
    fn discussion_case1() {
        test("11000111", 6)
    }

    #[test]
    fn discussion_case2() {
        test("11010011", 8)
    }

    #[test]
    fn my_extreme_ex1() {
        let mut input = [b'0'; 100_000];
        input[0] = b'1';
        input[2] = b'1';
        test(std::str::from_utf8(&input).unwrap(), 99_998 + 99_997)
    }

    #[test]
    fn my_extreme_ex2() {
        let mut input = [b'0'; 100_000];
        input[0] = b'1';
        input[2] = b'1';
        input[4] = b'1';
        test(
            std::str::from_utf8(&input).unwrap(),
            99_997 + 99_996 + 99_995,
        )
    }

    #[test]
    fn my_extreme_ex3() {
        let mut input = [b'1'; 100_000];
        input[0] = b'0';
        input[2] = b'0';
        test(std::str::from_utf8(&input).unwrap(), 1)
    }

    #[test]
    fn my_extreme_ex4() {
        let mut input = [b'1'; 100_000];
        input[0] = b'0';
        input[2] = b'0';
        input[4] = b'0';
        test(std::str::from_utf8(&input).unwrap(), 3)
    }

    #[test]
    fn my_extreme_ex5() {
        let mut input = [b'1'; 100_000];
        input[99_995] = b'0';
        input[99_997] = b'0';
        input[99_999] = b'0';
        test(
            std::str::from_utf8(&input).unwrap(),
            99_995 + 99_996 + 99_997,
        )
    }
}
