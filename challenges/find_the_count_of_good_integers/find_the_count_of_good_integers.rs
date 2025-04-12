// https://leetcode.com/problems/find-the-count-of-good-integers/

pub struct Solution;

// Naive direct sol'n (Too hard to make divisibility rules)
// impl Solution {
//     pub fn count_good_integers(n: i32, k: i32) -> i64 {
//         fn k_divisible(nonpalindromicnum: i32, digits: &[u8; 10], n: u8, k: u8) -> bool {
//             // Divisibility rules from https://www.ixl.com/math/lessons/divisibility-rules
//             // adapted for digit arrays and palindromic numbers (excluding leading zeros)
//             match k {
//                 1 => true,
//                 2 if nonpalindromicnum < 10 => nonpalindromicnum % 2 == 0,
//                 2 => digits[2] >= 2 || digits[4] >= 2 || digits[6] >= 2 || digits[8] >= 2,
//                 3 => digits.iter().sum::<u8>() % 3 == 0,
//                 4 if n == 1 => nonpalindromicnum % 4 == 0,
//                 4 if n == 2 => digits[4] >= 2 || digits[8] >= 2,
//                 4 if n == 3 => {
//                     let tens = digits.iter().position(|&d| d == 1).unwrap();
//                     let ones = digits.iter().position(|&d| d == 2).unwrap();
//                     (tens * 10 + ones) % 4 == 0
//                 }
//                 4 => {
//                     if digits[4] >= 4 || digits[8] >= 4 {
//                         true
//                     } else {
//                         let valid = digits.map(|d| d >= 2);
//                         ((valid[1] || valid[3] || valid[5] || valid[7] || valid[9])
//                             && (valid[2] || valid[6]))
//                             || ((valid[2] || valid[6]) && (valid[0] || valid[4] || valid[8]))
//                             || (valid[0] && (valid[4] || valid[8]))
//                             || (valid[4] && (valid[0] || valid[8]))
//                             || (valid[8] && (valid[0] || valid[4]))
//                     }
//                 }
//                 5 => digits[5] >= 2,
//                 6 => {
//                     k_divisible(nonpalindromicnum, digits, n, 2)
//                         && k_divisible(nonpalindromicnum, digits, n, 3)
//                 }
//                 7 if n == 1 => nonpalindromicnum == 7,
//                 7 if n == 2 => nonpalindromicnum == 77,
//                 7 if n == 3 => {
//                     let mut s = 0;
//                     for i in 0..digits.len() as u8 {
//                         if digits[i as usize] > 0 {
//                             s += i;
//                         }
//                     }
//                     s % 7 == 0
//                 }
//                 7 => todo!(),
//                 8 => todo!(),
//                 9 => digits.iter().sum::<u8>() % 9 == 0,
//                 _ => unreachable!(),
//             }
//         }
//     }
// }

// Focus on palindromic numbers of size n (Way too much implementation effort)
// impl Solution {
//     pub fn count_good_integers(n: i32, k: i32) -> i64 {
//         assert!(n >= 1);
//         assert!(n <= 10);
//         let n = n as u8;
//         assert!(k >= 1);
//         assert!(k <= 9);
//         let k = k as u8;
//         match (n, k) {
//             (1 | 2, 1) => (1..=9).count() as i64,
//             (1, k) => (1..=9).filter(|&num| num % k == 0).count() as i64,
//             (2, k) => (1..=9).filter(|&num| (11 * num) % k == 0).count() as i64,
//             (3, k) => {
//                 let mut count = 0;
//                 for inner in 0..=9 {
//                     for outer in 1..=9 {
//                         let num = 101 * outer + 10 * inner;
//                         if num % k as u16 == 0 {
//                             // 3 choose 1 ways to form the number if inner != outer, 1 way if inner == outer
//                             count += if inner != outer { 3 } else { 1 };
//                         }
//                     }
//                 }
//                 count
//             }
//             (4, k) => {
//                 let mut count = 0;
//                 for inner in 0..=9 {
//                     for outer in 1..=9 {
//                         let num = (1001 * outer + 110 * inner);
//                         if num % k as u16 == 0 {
//                             // 4 choose 2 ways to form the number if inner != outer, 1 way if inner == outer
//                             count += if inner != outer { 6 } else { 1 };
//                         }
//                     }
//                 }
//                 count
//             }
//             (5, k) => {
//                 let mut count: i64 = 0;
//                 for outer in 1..=9 {
//                     for mid in 0..=9 {
//                         for inner in 0..=9 {
//                             let num = 10001 * outer + 1010 * mid + 100 * inner;
//                             if num % k as u32 == 0 {
//                                 // any permutation of xyzyx is a valid way to reach this number
//                                 // those permutations are (5 choose 1) * (4 choose 2) = 5 * 6 = 30
//                                 // xxyyz, xxyzy, xxzyy, xzxyy, zxxyy
//                                 // xyxyz, xyxzy, xyzxy, xzyxy, zxyxy
//                                 // xyyxz, xyyzx, xyzyx, xzyyx, zxyyx
//                                 // yxyxz, yxyzx, yxzyx, yzxyx, zyxyx
//                                 // yyxxz, yyxzx, yyzxx, yzyxx, zyyxx
//                                 // yxxyz, yxxzy, yxzxy, yzxxy, zyxxy
//                                 count += 30;
//                             }
//                         }
//                     }
//                 }
//                 count
//             }
//             (6, k) => {
//                 let mut count = 0;
//                 for outer in 1..=9 {
//                     for mid in 0..=9 {
//                         for inner in 0..=9 {
//                             let num = 100001 * outer + 10010 * mid + 1100 * inner;
//                             if num % k as u32 == 0 {
//                                 // any permutation of xyzzyx is a valid way to reach this number
//                                 // those permutations are (6 choose 2) * (4 choose 2) = 6*6 = 36
//                                 count += 36;
//                             }
//                         }
//                     }
//                 }
//                 count
//             }
//             (7..=9, _) => todo!(),
//             (_, _) => unreachable!(),
//         }
//     }
// }

// Iterating left halfs of palindromes (First working sol'n, beats 100%, but I want faster)
// impl Solution {
//     pub fn count_good_integers(n: i32, k: i32) -> i64 {
//         const FACTORIAL_COUNT: usize = 11;
//         const FACTORIALS: [i64; FACTORIAL_COUNT] = {
//             let mut arr = [1; FACTORIAL_COUNT];
//             let mut i = 1;
//             while i < arr.len() {
//                 arr[i] = arr[i - 1] * i as i64;
//                 i += 1;
//             }
//             arr
//         };
//         assert!(n >= 1);
//         assert!(n <= 10);
//         let n = n as u8;
//         assert!(k >= 1);
//         assert!(k <= 9);
//         let k = k as u8;
//         match n {
//             1 => (1..=9).filter(|&num| num % k == 0).count() as i64,
//             2 => (1..=9).filter(|&num| (11 * num) % k == 0).count() as i64,
//             3 => {
//                 let mut count = 0;
//                 for inner in 0..=9 {
//                     for outer in 1..=9 {
//                         let num = 101 * outer + 10 * inner;
//                         if num % k as u16 == 0 {
//                             // 3 choose 1 ways to form the number if inner != outer, 1 way if inner == outer
//                             // unless inner == 0, in which case we can't have any leading 0s and lose one way
//                             count += if inner != outer {
//                                 3 - (inner == 0) as i64
//                             } else {
//                                 1
//                             };
//                         }
//                     }
//                 }
//                 count
//             }
//             4..=10 => {
//                 let left_side_len = (n + 1) / 2;
//                 let mut computed = std::collections::HashSet::new();
//                 let mut count = 0;
//                 for left_side in
//                     10u32.pow(left_side_len as u32 - 1)..10u32.pow(left_side_len as u32)
//                 {
//                     let mut s = left_side.to_string();
//                     let current_palindrome = {
//                         let siter_rev = s.chars().rev();
//                         let reversed: Vec<_> = if n % 2 == 0 {
//                             siter_rev.collect()
//                         } else {
//                             siter_rev.skip(1).collect()
//                         };
//                         s.extend(reversed);
//                         s
//                     };
//                     let current_palindrome_num = current_palindrome.parse::<u64>().unwrap();
//                     if current_palindrome_num % k as u64 != 0 {
//                         continue;
//                     }
//                     let mut digits = [0u8; 10];
//                     for &b in current_palindrome.as_bytes() {
//                         digits[(b - b'0') as usize] += 1;
//                     }
//                     if !computed.insert(digits) {
//                         continue;
//                     }
//                     let mut count_to_add = FACTORIALS[n as usize];
//                     for digit_count in digits {
//                         if digit_count > 0 {
//                             count_to_add /= FACTORIALS[digit_count as usize];
//                         }
//                     }
//                     count += count_to_add;
//                     if digits[0] <= 0 {
//                         continue;
//                     }
//                     let mut count_to_lose = FACTORIALS[n as usize - 1];
//                     digits[0] -= 1;
//                     for digit_count in digits {
//                         if digit_count > 0 {
//                             count_to_lose /= FACTORIALS[digit_count as usize];
//                         }
//                     }
//                     count -= count_to_lose;
//                 }
//                 count
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// Optimization: Reuse the same string buffers for each iteration
impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        const FACTORIAL_COUNT: usize = 11;
        const FACTORIALS: [i64; FACTORIAL_COUNT] = {
            let mut arr = [1; FACTORIAL_COUNT];
            let mut i = 1;
            while i < arr.len() {
                arr[i] = arr[i - 1] * i as i64;
                i += 1;
            }
            arr
        };
        assert!(n >= 1);
        assert!(n <= 10);
        let n = n as u8;
        assert!(k >= 1);
        assert!(k <= 9);
        let k = k as u8;
        match n {
            1 => (1..=9).filter(|&num| num % k == 0).count() as i64,
            2 => (1..=9).filter(|&num| (11 * num) % k == 0).count() as i64,
            3 => {
                let mut count = 0;
                for inner in 0..=9 {
                    for outer in 1..=9 {
                        let num = 101 * outer + 10 * inner;
                        if num % k as u16 == 0 {
                            // 3 choose 1 ways to form the number if inner != outer, 1 way if inner == outer
                            // unless inner == 0, in which case we can't have any leading 0s and lose one way
                            count += if inner != outer {
                                3 - (inner == 0) as i64
                            } else {
                                1
                            };
                        }
                    }
                }
                count
            }
            4..=10 => {
                use std::fmt::Write;
                let left_side_len = (n + 1) / 2;
                let mut count = 0;
                let mut palindrome_buffer = String::with_capacity(n as usize);
                let mut rev_buffer = Vec::with_capacity(n as usize);
                let num_range_start = 10u32.pow(left_side_len as u32 - 1);
                let num_range_end = 10u32.pow(left_side_len as u32);
                let mut computed = std::collections::HashSet::with_capacity(
                    (num_range_end - num_range_start) as usize,
                );
                for left_side in num_range_start..num_range_end {
                    palindrome_buffer.clear();
                    write!(&mut palindrome_buffer, "{}", left_side).expect("Write err");
                    let current_palindrome = {
                        let siter_rev = palindrome_buffer.as_bytes().iter().copied().rev();
                        rev_buffer.clear();
                        if n % 2 == 0 {
                            rev_buffer.extend(siter_rev);
                        } else {
                            rev_buffer.extend(siter_rev.skip(1));
                        }
                        palindrome_buffer
                            .push_str(unsafe { std::str::from_utf8_unchecked(&rev_buffer) });
                        &palindrome_buffer
                    };
                    let current_palindrome_num = current_palindrome.parse::<u64>().unwrap();
                    if current_palindrome_num % k as u64 != 0 {
                        continue;
                    }
                    let mut digits = [0u8; 10];
                    for &b in current_palindrome.as_bytes() {
                        digits[(b - b'0') as usize] += 1;
                    }
                    if !computed.insert(digits) {
                        continue;
                    }
                    let mut count_to_add = FACTORIALS[n as usize];
                    for digit_count in digits {
                        if digit_count > 0 {
                            count_to_add /= FACTORIALS[digit_count as usize];
                        }
                    }
                    count += count_to_add;
                    if digits[0] <= 0 {
                        continue;
                    }
                    let mut count_to_lose = FACTORIALS[n as usize - 1];
                    digits[0] -= 1;
                    for digit_count in digits {
                        if digit_count > 0 {
                            count_to_lose /= FACTORIALS[digit_count as usize];
                        }
                    }
                    count -= count_to_lose;
                }
                count
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u8, k: u8, expected: i64) {
        assert!(n >= 1);
        assert!(n <= 10);
        assert!(k >= 1);
        assert!(k <= 9);
        assert_eq!(Solution::count_good_integers(n as i32, k as i32), expected);
    }

    #[test]
    fn ex1() {
        // Good integers: 505, 515, 525, 535, 545, 555, 565, 575, 585, 595, 551, 552, 553, 554, 556, 557, 558, 559, 155, 255, 355, 455, 655, 755, 855, 955, 550
        test(3, 5, 27)
    }

    #[test]
    fn ex2() {
        test(1, 4, 2)
    }

    #[test]
    fn ex3() {
        test(5, 6, 2468)
    }

    #[test]
    fn my_simple_stuff_1() {
        test(1, 1, 9); // 1, 2, 3, 4, 5, 6, 7, 8, 9
        test(1, 2, 1 + 1 + 1 + 1); // 2, 4, 6, 8
        test(1, 3, 3); // 3, 6, 9
        test(1, 4, 2); // 4, 8
        test(1, 5, 1); // 5
        test(1, 6, 1); // 6
        test(1, 7, 1); // 7
        test(1, 8, 1); // 8
        test(1, 9, 1); // 9
    }

    #[test]
    fn my_simple_stuff_2() {
        test(2, 1, 9); // 11, 22, 33, 44, 55, 66, 77, 88, 99
        test(2, 2, 4); // 22, 44, 66, 88
        test(2, 3, 3); // 33, 66, 99
        test(2, 4, 2); // 44, 88
        test(2, 5, 1); // 55
        test(2, 6, 1); // 66
        test(2, 7, 1); // 77
        test(2, 8, 1); // 88
        test(2, 9, 1); // 99
    }

    #[test]
    fn my_simple_stuff_3() {
        test(3, 1, 243);
        test(3, 2, 108);
        test(3, 3, 69);
        test(3, 4, 54);
        test(3, 5, 27);
        test(3, 6, 30);
        test(3, 7, 33);
        test(3, 8, 27);
        test(3, 9, 23);
    }

    #[test]
    fn my_simple_stuff_4() {
        test(4, 1, 252);
        test(4, 4, 98);
        test(4, 5, 52);
        test(4, 7, 76);
        test(4, 8, 52);
        test(4, 9, 28);
    }

    #[test]
    fn my_extreme_ex1() {
        test(10, 1, 41457024);
        test(10, 2, 39718144);
        test(10, 3, 13831104);
        test(10, 4, 37326452);
        test(10, 5, 19284856);
        test(10, 6, 13249798);
        test(10, 7, 40242031);
        test(10, 8, 35755906);
        test(10, 9, 4610368);
    }
}
