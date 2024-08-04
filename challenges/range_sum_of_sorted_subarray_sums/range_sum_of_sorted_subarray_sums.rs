// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
//         assert!(n >= 1);
//         let n = n as usize;
//         assert_eq!(nums.len(), n);
//         let mut sums = vec![];
//         for i in 0..n {
//             let mut sum = 0;
//             for j in i..n {
//                 sum += nums[j];
//                 sums.push(sum);
//             }
//         }
//         sums.sort_unstable();
//         let mut result = 0;
//         for i in left as usize - 1..right as usize {
//             result += sums[i];
//             result %= 1_000_000_007;
//         }
//         result
//     }
// }

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        assert!(n >= 1);
        let n = n as usize;
        assert_eq!(nums.len(), n);
        let mut sums = vec![];
        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += nums[j];
                sums.push(sum);
            }
        }
        let left_zero_idx = left as usize - 1;
        sums.select_nth_unstable(left_zero_idx);
        sums[left_zero_idx..].select_nth_unstable(right as usize - 1 - left_zero_idx);
        let mut result = 0;
        for i in left_zero_idx..right as usize {
            result += sums[i];
            result %= 1_000_000_007;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], left: u32, right: u32, expected: u32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);
        assert!(left >= 1);
        assert!(right >= 1);
        assert!(left <= right);
        assert!(right <= nums.len() as u32 * (nums.len() as u32 + 1) / 2);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert_eq!(
            Solution::range_sum(nums.to_vec(), nums.len() as i32, left as i32, right as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 4], 1, 5, 13)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4], 3, 4, 6)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3, 4], 1, 10, 50)
    }

    const DISCUSSION_EXAMPLE_1: &[i32] = &[
        92, 73, 84, 69, 50, 26, 39, 13, 57, 84, 82, 35, 31, 83, 34, 100, 40, 11, 48, 19, 58, 4, 50,
        83, 58, 7, 81, 100, 99, 37, 66, 80, 99, 55, 60, 79, 73, 96, 67, 25, 21, 1, 35, 14, 50, 91,
        33, 26, 63, 61, 34, 66, 52, 17, 78, 64, 35, 23, 89, 2, 27, 44, 34, 91, 91, 14, 85, 4, 56,
        40, 42, 80, 93, 25, 43, 53, 100, 17, 83, 16, 32, 46, 98, 93, 61, 57, 23, 88, 46, 55, 56,
        53, 47, 75, 14, 12, 72, 58, 17, 13, 2, 68, 90, 50, 84, 62, 31, 51, 82, 93, 13, 54, 8, 87,
        38, 77, 22, 39, 36, 68, 16, 72, 86, 87, 44, 1, 19, 1, 12, 44, 97, 44, 58, 26, 32, 48, 86,
        74, 60, 34, 70, 22, 58, 52, 2, 14, 1, 14, 43, 93, 27, 40, 70, 15, 65, 100, 39, 12, 14, 70,
        79, 37, 95, 8, 32, 15, 88, 22, 57, 22, 22, 60, 34, 89, 22, 57, 91, 74, 4, 72, 49, 3, 16,
        13, 40, 11, 81, 99, 49, 27, 33, 93, 50, 30, 60, 83, 77, 65, 72, 48, 24, 55, 82, 2, 76, 54,
        78, 30, 37, 45, 66, 41, 61, 81, 98, 95, 60, 96, 92, 33, 45, 35, 100, 21, 24, 29, 60, 42,
        83, 3, 48, 61, 80, 90, 37, 87, 32, 3, 14, 59, 94, 67, 24, 26, 50, 90, 78, 43, 71, 18, 5,
        63, 53, 60, 57, 48, 2, 26, 73, 27, 9, 63, 17, 85, 12, 93, 67, 89, 86, 71, 52, 79, 36, 27,
        11, 55, 3, 70, 62, 92, 96, 44, 95, 1, 26, 79, 38, 98, 99, 54, 58, 91, 66, 90, 37, 9, 5, 12,
        45, 49, 83, 53, 49, 83, 90, 98, 82, 99, 30, 37, 63, 52, 71, 1, 81, 56, 82, 37, 87, 63, 20,
        52, 87, 56, 51, 92, 99, 76, 93, 77, 39, 61, 71, 48, 39, 54, 75, 83, 99, 22, 45, 5, 55, 57,
        57, 40, 9, 88, 30, 43, 30, 49, 4, 80, 44, 20, 24, 64, 9, 100, 13, 23, 78, 5, 30, 84, 23, 2,
        84, 67, 87, 56, 87, 56, 23, 38, 51, 57, 40, 69, 5, 21, 62, 21, 14, 85, 15, 61, 94, 37, 94,
        40, 39, 51, 5, 76, 35, 43, 98, 60, 9, 8, 98, 84, 92, 38, 29, 59, 82, 40, 71, 27, 49, 9, 55,
        6, 6, 12, 48, 1, 38, 40, 8, 1, 5, 80, 6, 20, 77, 22, 61, 84, 11, 75, 18, 3, 96, 72, 18, 14,
        10, 91, 9, 78, 100, 85, 15, 26, 13, 63, 56, 11, 15, 61, 92, 58, 65, 94, 22, 70, 14, 18, 48,
        94, 55, 66, 17, 11, 81, 65, 5, 94, 71, 32, 13, 94, 49, 48, 68, 42, 72, 90, 74, 70, 51, 40,
        31, 73, 89, 12, 20, 78, 52, 100, 47, 95, 37, 46, 65, 48, 3, 2, 1, 19, 33, 95, 88, 89, 27,
        84, 18, 24, 15, 63, 95, 62, 86, 17, 36, 34, 18, 6, 23, 85, 45, 96, 42, 26, 81, 13, 65, 64,
        19, 81, 33, 42, 25, 82, 15, 80, 86, 89, 32, 46, 52, 99, 14, 36, 100, 50, 72, 46, 5, 82, 43,
        30, 51, 5, 97, 54, 69, 35, 100, 21, 34, 6, 96, 35, 93, 11, 43, 84, 54, 1, 100, 87, 35, 30,
        84, 100, 91, 23, 33, 48, 68, 82, 15, 22, 64, 26, 73, 79, 76, 10, 53, 37, 89, 87, 6, 34, 1,
        4, 78, 21, 43, 23, 52, 30, 75, 71, 26, 5, 83, 70, 97, 25, 59, 70, 85, 58, 76, 80, 43, 45,
        47, 15, 99, 3, 54, 47, 24, 62, 30, 93, 88, 94, 82, 96, 93, 81, 50, 98, 77, 2, 86, 25, 79,
        96, 64, 53, 97, 51, 21, 20, 87, 55, 49, 57, 95, 93, 83, 96, 28, 33, 9, 90, 21, 73, 1, 39,
        96, 24, 13, 20, 33, 23, 51, 33, 67, 83, 54, 42, 69, 94, 55, 63, 60, 16, 33, 78, 41, 33, 33,
        93, 37, 49, 78, 66, 3, 64, 83, 66, 65, 26, 55, 35, 94, 84, 98, 53, 93, 5, 3, 54, 72, 11,
        36, 92, 4, 4, 64, 87, 50, 69, 45, 71, 61, 68, 57, 72, 96, 94, 91, 61, 75, 24, 76, 97, 4,
        78, 46, 27, 40, 56, 9, 29, 13, 22, 78, 59, 81, 17, 64, 41, 57, 89, 20, 63, 33, 63, 44, 91,
        68, 93, 29, 100, 6, 47, 66, 34, 51, 69, 84, 77, 13, 40, 17, 65, 18, 19, 9, 44, 25, 6, 4,
        63, 61, 10, 74, 68, 7, 18, 81, 60, 73, 18, 73, 34, 11, 39, 46, 17, 60, 48, 6, 43, 14, 13,
        80, 87, 15, 40, 85, 40, 75, 47, 24, 99, 1, 85, 97, 61, 52, 52, 82, 28, 12, 54, 35, 63, 56,
        86, 64, 49, 19, 55, 27, 98, 55, 81, 99, 25, 35, 10, 18, 60, 42, 47, 38, 58, 44, 32, 21, 60,
        17, 91, 44, 29, 64, 20, 13, 69, 76, 78, 57, 94, 41, 23, 12, 85, 41, 21, 35, 46, 85, 92, 15,
        15, 49, 42, 98, 42, 40, 43, 98, 1, 25, 85, 84, 39, 82, 24, 22, 72, 21, 61, 31, 11, 92, 68,
        34, 91, 14, 7, 49, 96, 12, 51, 60, 63, 85, 19, 91, 19, 8, 15, 20, 81, 16, 52, 78, 13, 37,
        11, 74, 75, 13, 28, 96, 39, 52, 23, 68, 58, 90, 35, 98, 44, 81, 23, 83, 52, 34, 23, 21, 93,
        11, 28, 82, 67, 99, 46, 87, 95, 59, 85, 12, 43, 67, 55, 39, 59, 8, 18, 45, 23, 64, 29, 30,
        9, 17, 98, 57, 56, 13, 47, 97, 29, 16, 42, 18, 26, 31, 79, 71, 41, 85, 64, 44, 76, 82, 62,
        65, 85, 35, 100, 38, 3, 80, 88,
    ];

    #[test]
    fn discussion_case1() {
        test(&DISCUSSION_EXAMPLE_1, 1, 5, 5)
    }

    #[test]
    fn discussion_case1_2() {
        test(&DISCUSSION_EXAMPLE_1, 5, 5000, 718041)
    }

    #[test]
    fn discussion_case2() {
        const N: usize = 1000;
        test(&[100; N], 1, 10000, 5516500)
    }

    #[test]
    fn failing_case1() {
        test(&[9, 3, 2, 2, 6, 9, 6, 6], 4, 8, 27)
    }
}
