// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
//         const MOD: i32 = 1_000_000_007;
//         let mut evens = 1;
//         let mut odds = 0;
//         let mut ans = 0;
//         let mut parity = true;
//         for i in arr {
//             parity ^= i % 2 == 1;
//             if parity {
//                 ans = (ans + odds) % MOD;
//                 evens += 1;
//             } else {
//                 ans = (ans + evens) % MOD;
//                 odds += 1;
//             }
//         }
//         ans
//     }
// }

//
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut evens = 1;
        let mut odds = 0;
        let mut ans = 0;
        let mut parity = true;
        for i in arr {
            parity ^= i & 1 != 0;
            if parity {
                ans = (ans + odds) % MOD;
                evens += 1;
            } else {
                ans = (ans + evens) % MOD;
                odds += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], expected: i32) {
        assert!(arr.len() >= 1);
        assert!(arr.len() <= 100_000);
        for &num in arr {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert_eq!(Solution::num_of_subarrays(arr.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 3, 5], 4)
    }

    #[test]
    fn ex2() {
        test(&[2, 4, 6], 0)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3, 4, 5, 6, 7], 16)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                26, 5, 44, 14, 74, 95, 33, 77, 22, 7, 85, 8, 2, 50, 31, 72, 75, 71, 74, 71, 91, 98,
                40, 87, 70, 69, 19, 16, 85, 83, 17, 51, 51, 48, 70, 14, 14, 17, 54, 44, 37, 45, 22,
                81, 31, 30, 65, 10, 53, 42, 57, 62, 57, 9, 14, 5, 81, 75, 12, 16, 59, 91, 20, 53,
                91, 15, 65, 61, 82, 7, 3, 57, 46, 74, 39, 11, 8, 86, 45, 44, 44, 78, 34, 52, 69,
                88, 83, 37, 62, 34, 37, 93, 30, 60, 95, 33, 71, 92, 90, 39, 58, 9, 67, 40, 47, 87,
                58, 93, 28, 38, 22, 55, 29, 15, 1, 48, 26, 18, 12, 21, 10, 25, 58, 62, 22, 63, 85,
                78, 8, 13, 4, 55, 49, 25, 54, 39, 14, 68, 41, 33, 56, 25, 25, 29, 23, 69, 45, 51,
                100, 57, 77, 23, 54, 11, 61, 21, 8, 64, 80, 5, 79, 70, 33, 62, 27, 11, 90, 24, 54,
                3, 8, 9, 79, 69, 31, 34, 51, 79, 87, 97, 67, 85, 70, 73, 3, 15, 36, 60, 69, 66, 16,
                22, 71, 43, 89, 57, 83, 54, 37, 49, 89, 71, 81, 88, 61, 19, 79, 70, 13, 15, 22, 20,
                83, 48, 79, 91, 86, 15, 33, 23, 2, 7, 78, 26, 94, 38, 37, 40, 30, 94, 3, 25, 94,
                70, 54, 100, 51, 69, 28, 93, 44, 79, 33, 37, 62, 5, 14, 75, 44, 32, 94, 23, 75, 88,
                79, 74, 78, 49, 75, 84, 57, 82, 80, 10, 23, 31, 48, 96, 48, 83, 90, 59, 39, 79, 7,
                63, 89, 21, 39, 70, 36, 84, 69, 90, 74, 37, 43, 22, 88, 97, 58, 49, 19, 62, 11, 90,
                14, 40, 15, 32, 52, 75, 79, 1, 12, 66, 71, 28, 49, 13, 21, 63, 13, 25, 33, 48, 27,
                18, 49, 7, 60, 17, 39, 74, 52, 91, 18, 8, 29, 58, 91, 78, 45, 15, 90, 12, 12, 8,
                51, 12, 63, 26,
            ],
            29392,
        )
    }

    #[test]
    fn discussion_case2() {
        test(&[100, 100, 99, 99], 4)
    }

    #[test]
    fn my_extreme_ex1() {
        let input = vec![100; 100_000];
        test(&input, 0)
    }

    #[test]
    fn my_extreme_ex2() {
        let input = vec![99; 100_000];
        test(&input, 500049986)
    }
}
