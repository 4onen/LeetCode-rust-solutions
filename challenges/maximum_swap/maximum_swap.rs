// https://leetcode.com/problems/maximum-swap/

pub struct Solution;

// O(n^2) solution (lol)
// impl Solution {
//     pub fn maximum_swap(mut num: i32) -> i32 {
//         let mut digits = Vec::with_capacity(9);
//         while num > 0 {
//             digits.push(num % 10);
//             num /= 10;
//         }
//         digits.reverse();
//         let mut max = digits.clone();
//         for i in 0..digits.len() {
//             for j in (i + 1)..digits.len() {
//                 digits.swap(i, j);
//                 if digits > max {
//                     max = digits.clone();
//                 }
//                 digits.swap(i, j);
//             }
//         }
//         max.into_iter().fold(0, |acc, x| acc * 10 + x)
//     }
// }

// Optimized solution
impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let original_num = num;
        let mut digits = Vec::with_capacity(9);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        let mut sorted_digits = digits.clone();
        sorted_digits.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
        let mut i = 0;
        loop {
            if i == digits.len() {
                return original_num;
            }
            if digits[i] != sorted_digits[i] {
                break;
            }
            i += 1;
        }
        let mut j = digits.len() - 1;
        while digits[j] != sorted_digits[i] {
            j -= 1;
        }
        digits.swap(i, j);
        digits.into_iter().fold(0, |acc, x| acc * 10 + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num: i32, expected: i32) {
        assert_eq!(Solution::maximum_swap(num), expected, "num: {}", num);
    }

    #[test]
    fn ex1() {
        test(2736, 7236);
    }

    #[test]
    fn ex2() {
        test(9973, 9973);
    }

    #[test]
    fn copilot_case1() {
        test(98368, 98863);
    }

    #[test]
    fn copilot_case2() {
        test(1993, 9913);
    }

    #[test]
    fn discussion_case1() {
        test(13899, 93891);
    }

    #[test]
    fn discussion_case2() {
        test(3, 3);
    }

    #[test]
    fn discussion_case3() {
        test(6687, 8667);
    }

    #[test]
    fn discussion_case4() {
        test(56867898, 96867858);
    }

    #[test]
    fn discussion_case5() {
        test(98004365, 98604305);
    }

    #[test]
    fn discussion_case6() {
        test(98765432, 98765432);
    }

    #[test]
    fn discussion_case7() {
        test(98800435, 98850430);
    }

    #[test]
    fn my_extreme_ex1() {
        test(100_000_000, 100_000_000);
    }
}
