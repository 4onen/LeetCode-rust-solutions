// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/

pub struct Solution;

// O(n^2) with recomputation every time
// impl Solution {
//     pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
//         const MAX_DIGITS: u8 = 9;
//         fn digits_of(mut x: i32) -> [i8; MAX_DIGITS as usize] {
//             let mut digits = [-1; MAX_DIGITS as usize];
//             let mut i = 0;
//             while x > 0 {
//                 digits[i] = (x % 10) as i8;
//                 x /= 10;
//                 i += 1;
//             }
//             digits.reverse();
//             // Now we need to find the first not -1 element
//             let p = digits.iter().position(|&x| x != -1).unwrap() as usize;
//             // Rotate the array to the left by `p` positions
//             // Because the elements we're moving to the end are all -1 this
//             // is implemented as a memmove plus a memset
//             digits.copy_within(p.., 0);
//             digits[MAX_DIGITS as usize - p..].fill(-1);
//             digits
//         }
//         let mut longest_common_prefix = 0;
//         for x in arr1 {
//             for &y in &arr2 {
//                 let digits1 = digits_of(x);
//                 let digits2 = digits_of(y);
//                 let mut i = 0u8;
//                 while i < MAX_DIGITS
//                     && digits1[i as usize] >= 0
//                     && digits2[i as usize] >= 0
//                     && digits1[i as usize] == digits2[i as usize]
//                 {
//                     i += 1;
//                 }
//                 dbg!(&digits1, &digits2, i);
//                 longest_common_prefix = std::cmp::max(longest_common_prefix, i as i32);
//             }
//         }
//         longest_common_prefix
//     }
// }

// O(n log n) sol'n with sorting
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        const MAX_DIGITS: u8 = 9;
        fn digits_of(mut x: i32) -> [i8; MAX_DIGITS as usize] {
            let mut digits = [-1; MAX_DIGITS as usize];
            let mut i = 0;
            while x > 0 {
                digits[i] = (x % 10) as i8;
                x /= 10;
                i += 1;
            }
            digits.reverse();
            // Now we need to find the first not -1 element
            let p = digits.iter().position(|&x| x != -1).unwrap() as usize;
            // Rotate the array to the left by `p` positions
            // Because the elements we're moving to the end are all -1 this
            // is implemented as a memmove plus a memset
            digits.copy_within(p.., 0);
            digits[MAX_DIGITS as usize - p..].fill(-1);
            digits
        }
        let mut arr1 = arr1.into_iter().map(digits_of).collect::<Vec<_>>();
        let mut arr2 = arr2.into_iter().map(digits_of).collect::<Vec<_>>();
        arr1.sort_unstable();
        arr2.sort_unstable();
        let mut longest_common_prefix = 0;
        let mut arr1_iter = arr1.into_iter().peekable();
        let mut arr2_iter = arr2.into_iter().peekable();
        while let (Some(digits1), Some(digits2)) = (arr1_iter.peek(), arr2_iter.peek()) {
            let mut i = 0u8;
            while i < MAX_DIGITS
                && digits1[i as usize] >= 0
                && digits2[i as usize] >= 0
                && digits1[i as usize] == digits2[i as usize]
            {
                i += 1;
            }
            longest_common_prefix = std::cmp::max(longest_common_prefix, i as i32);
            if digits1 < digits2 {
                arr1_iter.next();
            } else {
                arr2_iter.next();
            }
        }
        longest_common_prefix
    }
}

// Sorting sol'n refactored to use indexes rather than iterators
// impl Solution {
//     pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
//         const MAX_DIGITS: u8 = 9;
//         fn digits_of(mut x: i32) -> [i8; MAX_DIGITS as usize] {
//             let mut digits = [-1; MAX_DIGITS as usize];
//             let mut i = 0;
//             while x > 0 {
//                 digits[i] = (x % 10) as i8;
//                 x /= 10;
//                 i += 1;
//             }
//             digits.reverse();
//             // Now we need to find the first not -1 element
//             let p = digits.iter().position(|&x| x != -1).unwrap() as usize;
//             // Rotate the array to the left by `p` positions
//             // Because the elements we're moving to the end are all -1 this
//             // is implemented as a memmove plus a memset
//             digits.copy_within(p.., 0);
//             digits[MAX_DIGITS as usize - p..].fill(-1);
//             digits
//         }
//         assert!(arr1.len() <= 50_000);
//         assert!(arr2.len() <= 50_000);
//         let mut arr1 = arr1.into_iter().map(digits_of).collect::<Vec<_>>();
//         let mut arr2 = arr2.into_iter().map(digits_of).collect::<Vec<_>>();
//         arr1.sort_unstable();
//         arr2.sort_unstable();
//         let mut longest_common_prefix = 0;
//         let mut idx1 = 0u16;
//         let mut idx2 = 0u16;
//         while idx1 < arr1.len() as u16 && idx2 < arr2.len() as u16 {
//             let mut i = 0u8;
//             while i < MAX_DIGITS
//                 && arr1[idx1 as usize][i as usize] >= 0
//                 && arr2[idx2 as usize][i as usize] >= 0
//                 && arr1[idx1 as usize][i as usize] == arr2[idx2 as usize][i as usize]
//             {
//                 i += 1;
//             }
//             longest_common_prefix = std::cmp::max(longest_common_prefix, i as i32);
//             if arr1[idx1 as usize] < arr2[idx2 as usize] {
//                 idx1 += 1;
//             } else {
//                 idx2 += 1;
//             }
//         }
//         longest_common_prefix
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr1: &[i32], arr2: &[i32], expected: i32) {
        assert!(arr1.len() >= 1);
        assert!(arr2.len() >= 1);
        assert!(arr1.len() <= 50_000);
        assert!(arr2.len() <= 50_000);
        for &x in arr1 {
            assert!(x >= 1);
            assert!(x <= 100_000_000);
        }
        for &x in arr2 {
            assert!(x >= 1);
            assert!(x <= 100_000_000);
        }
        assert_eq!(
            Solution::longest_common_prefix(arr1.to_vec(), arr2.to_vec()),
            expected
        );
        // Reversing the arguments should not change the result
        assert_eq!(
            Solution::longest_common_prefix(arr2.to_vec(), arr1.to_vec()),
            expected
        );
        // Removing elements with digits less than `expected` should not change the result
        if expected > 0 {
            assert_eq!(
                Solution::longest_common_prefix(
                    arr1.iter()
                        .copied()
                        .filter(|&x| x >= 10_i32.pow((expected - 1) as u32))
                        .collect(),
                    arr2.iter()
                        .copied()
                        .filter(|&x| x >= 10_i32.pow((expected - 1) as u32))
                        .collect()
                ),
                expected
            )
        }
    }

    #[test]
    fn ex1() {
        test(&[1, 10, 100], &[1000], 3)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3], &[4, 4, 4], 0)
    }

    #[test]
    fn discussion_case1() {
        test(&[98765432], &[987], 3)
    }

    #[test]
    fn discussion_case2() {
        test(&[12, 3456], &[12345, 456789, 12], 2)
    }

    #[test]
    fn discussion_case3() {
        test(&[10], &[17, 11], 1)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[100_000_000; 50_000], &[100_000_000; 50_000], 9)
    }

    #[test]
    fn my_extreme_ex2() {
        test(&[100_000_000; 50_000], &[10_000_000; 50_000], 8)
    }

    #[test]
    fn my_extreme_ex3() {
        test(&[100_000_000; 50_000], &[1; 50_000], 1)
    }

    #[test]
    fn my_extreme_ex4() {
        let mut arr2 = vec![1; 50_000];
        arr2[0] = 100_000_000;
        for i in 1..50_000 {
            arr2[i] = arr2[i - 1] - 1;
        }
        test(&[100_000_000; 50_000], &arr2, 9)
    }
}
