// https://leetcode.com/problems/count-the-number-of-powerful-integers/

pub struct Solution;

// Naive sol'n - simply count over range
// impl Solution {
//     pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
//         let mut count = 0;
//         let limit = limit as u8 + b'0';
//         for num in start..=finish {
//             let num = num.to_string();
//             if num.as_bytes().iter().all(|&b| b <= limit) && num.ends_with(&s) {
//                 count += 1;
//             }
//         }
//         count
//     }
// }

// Slight improvement: unsigned integer types
// impl Solution {
//     pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
//         assert!(start >= 1);
//         assert!(finish >= start);
//         let start = start as u64;
//         let finish = finish as u64;
//         let s_num: u64 = s.parse().unwrap();
//         if s_num > finish {
//             return 0;
//         }
//         if s_num == finish {
//             return 1;
//         }
//         let limit = limit as u8 + b'0';
//         let mut count = 0;
//         for num in std::cmp::max(start, s_num)..=finish {
//             let num = num.to_string();
//             if num.as_bytes().iter().all(|&b| b <= limit) && num.ends_with(&s) {
//                 count += 1;
//             }
//         }
//         count
//     }
// }

// Improvement: iterate prefixes of s (couple off-by-one issues)
// impl Solution {
//     pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
//         assert!(start >= 1);
//         assert!(finish >= start);
//         let start = start as u64;
//         let finish = finish as u64;
//         let mut count = 0;
//         let limit = limit as u8 + b'0';
//         let s_num: u64 = s.parse().unwrap();
//         if s_num > finish {
//             return 0;
//         }
//         if s_num == finish {
//             return 1;
//         }
//         let s_pow = 10u64.pow(s.len() as u32);
//         let prefix_start = if start < s_num {
//             0
//         } else if start.ilog10() as usize <= s.len() {
//             1
//         } else {
//             start / s_pow
//         };
//         let prefix_end_remainder = finish % s_pow;
//         let prefix_end = finish / s_pow - (prefix_end_remainder < s_num) as u64;
//         for num in prefix_start..=prefix_end {
//             let num = num.to_string();
//             if num.as_bytes().iter().all(|&b| b <= limit) {
//                 count += 1;
//             }
//         }
//         count
//     }
// }

// Optimized: Make 15-digit buffers, count in there (Not fast enough)
// impl Solution {
//     pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
//         const POSITIONS: usize = 16;
//         type Buf = [u8; POSITIONS];
//         const fn num_2_buf(mut num: u64) -> Buf {
//             let mut buf = [0u8; POSITIONS];
//             let mut i = POSITIONS;
//             while num > 0 {
//                 i = i.saturating_sub(1);
//                 buf[i] = (num % 10) as u8;
//                 num /= 10;
//             }
//             buf
//         }
//         const fn str_2_buf(s: &str) -> Buf {
//             let s = s.as_bytes();
//             let mut buf = [0u8; POSITIONS];
//             let mut i = POSITIONS;
//             let mut j = s.len();
//             while j > 0 {
//                 j -= 1;
//                 i -= 1;
//                 buf[i] = s[j as usize] - b'0';
//             }
//             buf
//         }
//         // const fn buf_leading_zeros(buf: &Buf) -> u8 {
//         //     let mut i = 0;
//         //     while i < buf.len() as u8 && buf[i as usize] == 0 {
//         //         i += 1;
//         //     }
//         //     i
//         // }
//         fn inc_buf(buf: &mut [u8], limit: u8) {
//             let Some(last_digit) = buf.last_mut() else {
//                 unreachable!()
//             };
//             if *last_digit < limit {
//                 *last_digit += 1;
//             } else {
//                 *last_digit = 0;
//                 let l = buf.len() - 1;
//                 return inc_buf(&mut buf[..l], limit);
//             }
//         }
//         assert!(start >= 1);
//         assert!(finish >= start);
//         assert!(limit >= 1);
//         assert!(limit <= 9);
//         let limit = limit as u8;
//         let start = num_2_buf(start as u64);
//         let finish = num_2_buf(finish as u64);
//         let s_buf = str_2_buf(&s);
//         let s_leading_zeros = s_buf.len() - s.len();
//         if s_buf > finish {
//             0
//         } else if s_buf == finish {
//             1
//         } else {
//             let mut count = 0;
//             let mut b = start;
//             while b[s_leading_zeros..] != s_buf[s_leading_zeros..] {
//                 inc_buf(&mut b[s_leading_zeros - 1..], limit);
//             }
//             while b <= finish {
//                 count += 1;
//                 inc_buf(&mut b[..s_leading_zeros], limit);
//             }
//             count
//         }
//     }
// }

// Calculate both ends of buffer to hit prefix, then subtract
// (Giving up on this approach because it's too annoying to
// make the start and ends legal wrt the limit)
// impl Solution {
//     pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
//         const POSITIONS: usize = 16;
//         type Buf = [u8; POSITIONS];
//         const fn num_2_buf(mut num: u64) -> Buf {
//             let mut buf = [0u8; POSITIONS];
//             let mut i = POSITIONS;
//             while num > 0 {
//                 i = i.saturating_sub(1);
//                 buf[i] = (num % 10) as u8;
//                 num /= 10;
//             }
//             buf
//         }
//         const fn str_2_buf(s: &str) -> Buf {
//             let s = s.as_bytes();
//             let mut buf = [0u8; POSITIONS];
//             let mut i = POSITIONS;
//             let mut j = s.len();
//             while j > 0 {
//                 j -= 1;
//                 i -= 1;
//                 buf[i] = s[j as usize] - b'0';
//             }
//             buf
//         }
//         const fn buf_leading_zeros(buf: &[u8]) -> u8 {
//             let mut i = 0;
//             while i < buf.len() as u8 && buf[i as usize] == 0 {
//                 i += 1;
//             }
//             i
//         }
//         fn inc_buf(buf: &mut [u8], limit: u8) {
//             let Some(last_digit) = buf.last_mut() else {
//                 unreachable!()
//             };
//             if *last_digit < limit {
//                 *last_digit += 1;
//             } else {
//                 *last_digit = 0;
//                 let l = buf.len() - 1;
//                 return inc_buf(&mut buf[..l], limit);
//             }
//         }
//         fn dec_buf(buf: &mut [u8], limit: u8) {
//             let Some(last_digit) = buf.last_mut() else {
//                 unreachable!()
//             };
//             if *last_digit > 0 {
//                 if *last_digit > limit {
//                     *last_digit = limit;
//                 } else {
//                     *last_digit -= 1;
//                 }
//             } else {
//                 *last_digit = limit;
//                 let l = buf.len() - 1;
//                 return dec_buf(&mut buf[..l], limit);
//             }
//         }
//         fn buf_2_num(buf: &[u8], base: u8) -> u64 {
//             // Base is some number from 2 to 10
//             let mut num = 0;
//             let mut i = buf_leading_zeros(buf);
//             let l = buf.len() as u8;
//             while i < l {
//                 num = (num * base as u64) + buf[i as usize] as u64;
//                 i += 1;
//             }
//             num
//         }
//         assert!(start >= 1);
//         assert!(finish >= start);
//         assert!(limit >= 1);
//         assert!(limit <= 9);
//         let limit = limit as u8;
//         let start = num_2_buf(start as u64);
//         let finish = num_2_buf(finish as u64);
//         let s_buf = str_2_buf(&s);
//         let s_leading_zeros = s_buf.len() - s.len();
//         if s_buf > finish {
//             0
//         } else if s_buf == finish {
//             1
//         } else {
//             let mut startb = start;
//             while startb[s_leading_zeros..] != s_buf[s_leading_zeros..] {
//                 inc_buf(&mut startb, limit);
//             }
//             let newstart = buf_2_num(&startb[..s_leading_zeros], limit + 1);
//             let mut endb = finish;
//             while endb[s_leading_zeros..] != s_buf[s_leading_zeros..] {
//                 dec_buf(&mut endb, limit);
//             }
//             let newend = buf_2_num(&endb[..s_leading_zeros], limit + 1);
//             if newend < newstart {
//                 0
//             } else {
//                 (newend - newstart + 1) as i64
//             }
//         }
//     }
// }

// Difference of ranges [1,start-1] and [1,finish]
impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        pub fn powerful_int_below(finish: &[u8], limit: u8, s: &[u8]) -> u64 {
            if finish.len() < s.len() {
                return 0;
            } else if finish.len() == s.len() {
                return (finish >= s) as u64;
            }
            let mut count: u64 = 0;
            let prefix_len = (finish.len() - s.len()) as u8;
            let finish_suffix = &finish[prefix_len as usize..];
            for i in 0..prefix_len {
                let digit = finish[i as usize] - b'0';
                if digit > limit {
                    count += ((limit + 1) as u64).pow(prefix_len as u32 - i as u32);
                    return count;
                }
                count += digit as u64 * ((limit + 1) as u64).pow(prefix_len as u32 - i as u32 - 1);
            }
            count += (finish_suffix >= s) as u64;
            count
        }
        let limit = limit as u8;
        (powerful_int_below(finish.to_string().as_bytes(), limit, &s.as_bytes())
            - powerful_int_below((start - 1).to_string().as_bytes(), limit, &s.as_bytes()))
            as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(start: u64, finish: u64, limit: u8, s: &str, expected: i64) {
        assert!(start >= 1);
        assert!(finish >= start);
        assert!(finish <= 1_000_000_000_000_000);
        assert!(limit >= 1);
        assert!(limit <= 9);
        assert!(s.len() >= 1);
        assert!(s.len() <= finish.ilog10() as usize + 1);
        let mut seen_nonzero = false;
        for &b in s.as_bytes() {
            assert!(b >= b'0');
            assert!(b - b'0' <= limit as u8);
            assert!(b <= b'9');
            if b != b'0' {
                seen_nonzero = true;
            } else {
                assert!(seen_nonzero);
            }
        }
        assert_eq!(
            Solution::number_of_powerful_int(
                start as i64,
                finish as i64,
                limit as i32,
                s.to_owned()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(1, 6000, 4, "124", 5)
    }

    #[test]
    fn ex2() {
        test(15, 215, 6, "10", 2)
    }

    #[test]
    fn ex3() {
        test(1000, 2000, 4, "3000", 0)
    }

    #[test]
    fn myex1() {
        test(12, 31, 2, "1", 1)
    }

    #[test]
    fn myex2() {
        test(1, 999999990, 2, "1", 6561)
    }

    #[test]
    fn myex3() {
        test(3, 15, 2, "12", 1)
    }

    #[test]
    fn myex3_1() {
        test(31, 51, 2, "2", 0)
    }

    #[test]
    fn myex4() {
        test(3111111, 51111111, 2, "11111111", 1)
    }

    #[test]
    fn discussion_case1() {
        test(20, 1159, 5, "20", 8)
    }

    #[test]
    fn discussion_case2() {
        test(1, 971, 9, "72", 9)
    }

    #[test]
    fn discussion_case3() {
        test(1, 6000, 4, "124", 5)
    }

    #[test]
    fn discussion_case4() {
        test(15398, 1424153842, 8, "101", 783790)
    }

    #[test]
    fn discussion_case5() {
        test(1, 2000, 8, "1", 162)
    }

    #[test]
    fn discussion_case6() {
        test(15, 1440, 5, "11", 10)
    }

    #[test]
    fn discussion_case7() {
        test(1114, 1864854501, 7, "26", 4194295)
    }

    #[test]
    fn discussion_case7_1() {
        test(1114, 1864854501, 7, "6", 33554359)
    }

    #[test]
    fn discussion_case7_2() {
        test(1114, 1864854501, 7, "16", 4194295)
    }

    #[test]
    fn discussion_case8() {
        test(123546, 32486458654, 4, "1", 7030275)
    }

    #[test]
    fn discussion_case9() {
        test(1, 1000000000000000, 5, "1000000000000000", 1)
    }

    #[test]
    fn my_extreme_ex1_1() {
        let tgt = 1_000;
        test(1, tgt, 9, "1", tgt as i64 / 10)
    }

    #[test]
    fn my_extreme_ex1_2() {
        let tgt = 1_000_000;
        test(1, tgt, 9, "1", tgt as i64 / 10)
    }

    #[test]
    fn my_extreme_ex1_3() {
        let tgt = 1_000_000_000;
        test(1, tgt, 9, "1", tgt as i64 / 10)
    }

    #[test]
    fn my_extreme_ex1_4() {
        let tgt = 1_000_000_000_000;
        test(1, tgt, 9, "1", tgt as i64 / 10)
    }

    #[test]
    fn my_extreme_ex1_5() {
        let tgt = 1_000_000_000_000_000;
        test(1, tgt, 9, "1", tgt as i64 / 10)
    }
}
