// https://leetcode.com/problems/roman-to-integer/

pub struct Solution;

// For loop solution
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut prev = 0;
        for c in s.bytes().rev() {
            let curr = match c {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => unreachable!(),
            };
            if curr < prev {
                sum -= curr;
            } else {
                sum += curr;
            }
            prev = curr;
        }
        sum
    }
}

// Iterator scan solution
// impl Solution {
//     pub fn roman_to_int(s: String) -> i32 {
//         s.bytes()
//             .rev()
//             .map(|c| match c {
//                 b'I' => 1,
//                 b'V' => 5,
//                 b'X' => 10,
//                 b'L' => 50,
//                 b'C' => 100,
//                 b'D' => 500,
//                 b'M' => 1000,
//                 _ => unreachable!(),
//             })
//             .scan(0, |prev, x| {
//                 let res = if x < *prev { -x } else { x };
//                 *prev = x;
//                 Some(res)
//             })
//             .sum()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn failing_case() {
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
    }
}
