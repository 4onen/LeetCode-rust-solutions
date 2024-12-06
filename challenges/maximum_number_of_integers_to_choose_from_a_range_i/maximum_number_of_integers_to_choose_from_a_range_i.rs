// https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/

pub struct Solution;

// Initial sol'n with duplicate banned fix
// impl Solution {
//     pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
//         banned.sort_unstable();
//         let mut cnt = 0;
//         let mut sum = 0;
//         let mut banned_idx = 0;
//         for i in 1..=n {
//             if banned_idx < banned.len() && i == banned[banned_idx] {
//                 while banned_idx < banned.len() && i == banned[banned_idx] {
//                     banned_idx += 1;
//                 }
//                 continue;
//             }
//             sum += i;
//             if sum > max_sum {
//                 break;
//             }
//             cnt += 1;
//         }
//         cnt
//     }
// }

// Optimized sol'n - u16s and hashset banned
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        assert!(n >= 1);
        assert!(n <= 10_000);
        let n = n as u16;
        let banned: std::collections::HashSet<u16> = banned.iter().map(|&x| x as u16).collect();
        let mut cnt = 0;
        let mut sum = 0;
        for i in 1..=n {
            if banned.contains(&i) {
                continue;
            }
            sum += i as i32;
            if sum > max_sum {
                break;
            }
            cnt += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(banned: &[i32], n: i32, max_sum: i32, expected: i32) {
        assert!(banned.len() >= 1);
        assert!(banned.len() <= 10_000);
        assert!(n >= 1);
        assert!(n <= 10_000);
        assert!(max_sum >= 1);
        assert!(max_sum <= 1_000_000_000);
        assert_eq!(Solution::max_count(banned.to_vec(), n, max_sum), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 6, 5], 5, 6, 2);
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4, 5, 6, 7], 8, 1, 0);
    }

    #[test]
    fn ex3() {
        test(&[11], 7, 50, 7);
    }

    #[test]
    fn failing_case1() {
        test(
            &[
                87, 193, 85, 55, 14, 69, 26, 133, 171, 180, 4, 8, 29, 121, 182, 78, 157, 53, 26, 7,
                117, 138, 57, 167, 8, 103, 32, 110, 15, 190, 139, 16, 49, 138, 68, 69, 92, 89, 140,
                149, 107, 104, 2, 135, 193, 87, 21, 194, 192, 9, 161, 188, 73, 84, 83, 31, 86, 33,
                138, 63, 127, 73, 114, 32, 66, 64, 19, 175, 108, 80, 176, 52, 124, 94, 33, 55, 130,
                147, 39, 76, 22, 112, 113, 136, 100, 134, 155, 40, 170, 144, 37, 43, 151, 137, 82,
                127, 73,
            ],
            1079,
            87,
            9,
        );
    }
}
