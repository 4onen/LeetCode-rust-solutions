// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
//         if bloom_day.len() < m as usize * k as usize {
//             return -1;
//         }
//         const fn pass(bloom_day: &[i32], m: i32, k: i32, day: i32) -> bool {
//             let mut bouquets = 0;
//             let mut flowers = 0;
//             let mut i = 0;
//             while i < bloom_day.len() {
//                 if bloom_day[i as usize] <= day {
//                     flowers += 1;
//                     if flowers >= k {
//                         flowers = 0;
//                         bouquets += 1;
//                         if bouquets >= m {
//                             return true;
//                         }
//                     }
//                 } else {
//                     flowers = 0;
//                 }
//                 i += 1;
//             }
//             false
//         }
//         let mut min_day = 1;
//         let mut max_day = *bloom_day.iter().max().unwrap();
//         while min_day < max_day {
//             let mid = min_day + ((max_day - min_day) >> 1);
//             if pass(&bloom_day, m, k, mid) {
//                 max_day = mid;
//             } else {
//                 min_day = mid + 1;
//             }
//         }
//         if pass(&bloom_day, m, k, min_day) {
//             min_day
//         } else {
//             -1
//         }
//     }
// }

// Optimized pass fn
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if bloom_day.len() < m as usize * k as usize {
            return -1;
        }
        const fn pass(bloom_day: &[i32], m: i32, k: i32, day: i32) -> bool {
            let mut bouquets = 0;
            let mut flowers = 0;
            let mut i = 0;
            while i < bloom_day.len() {
                if bloom_day[i as usize] <= day {
                    flowers += 1;
                } else {
                    bouquets += flowers / k;
                    if bouquets >= m {
                        return true;
                    }
                    flowers = 0;
                }
                i += 1;
            }
            bouquets += flowers / k;
            bouquets >= m
        }
        let mut min_day = 1;
        let mut max_day = *bloom_day.iter().max().unwrap();
        while min_day < max_day {
            let mid = min_day + ((max_day - min_day) >> 1);
            if pass(&bloom_day, m, k, mid) {
                max_day = mid;
            } else {
                min_day = mid + 1;
            }
        }
        if pass(&bloom_day, m, k, min_day) {
            min_day
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(bloom_day: &[i32], m: i32, k: i32, expected: i32) {
        assert!(bloom_day.len() > 0);
        assert!(bloom_day.len() <= 100_000);
        assert!(m > 0);
        assert!(m <= 1_000_000);
        assert!(k > 0);
        assert!((k as u32) < (bloom_day.len() as u32));
        for flower in bloom_day {
            assert!(*flower >= 1);
            assert!(*flower <= 1_000_000_000);
        }
        assert_eq!(Solution::min_days(bloom_day.to_vec(), m, k), expected);
    }

    #[test]
    fn ex1() {
        test(&[1,10,3,10,2], 3, 1, 3)
    }

    #[test]
    fn ex2() {
        test(&[1,10,3,10,2], 3, 2, -1)
    }

    #[test]
    fn ex3() {
        test(&[7,7,7,7,12,7,7], 2, 3, 12)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 2, 4, 2, 3, 4, 1], 2, 2, 3)
    }
}
