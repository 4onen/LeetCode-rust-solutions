// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_max_k(nums: Vec<i32>) -> i32 {
//         let mut negatives = std::collections::HashSet::<i32>::new();
//         let mut positives = std::collections::HashSet::<i32>::new();
//         let mut maxk = -1;
//         for num in nums {
//             if num == 0 {
//                 unreachable!();
//             } else if num < 0 {
//                 if num == i32::MIN {
//                     unreachable!();
//                 }
//                 let num = num.abs();
//                 negatives.insert(num);
//                 if positives.contains(&num) {
//                     if num > maxk {
//                         maxk = num;
//                     }
//                 }
//             } else {
//                 // num > 0
//                 positives.insert(num.abs());
//                 if negatives.contains(&num) {
//                     if num > maxk {
//                         maxk = num;
//                     }
//                 }
//             }
//         }
//         maxk
//     }
// }

// Bitset sol'n
// impl Solution {
//     pub fn find_max_k(nums: Vec<i32>) -> i32 {
//         let mut found = [0u8; 1001];
//         let mut maxk = -1;
//         for num in nums {
//             if num == 0 {
//                 unreachable!();
//             } else if num < 0 {
//                 let num = -num;
//                 found[num as usize] |= 1;
//                 if found[num as usize] == 3 {
//                     if num > maxk {
//                         maxk = num;
//                     }
//                 }
//             } else {
//                 // num > 0
//                 found[num as usize] |= 2;
//                 if found[num as usize] == 3 {
//                     if num > maxk {
//                         maxk = num;
//                     }
//                 }
//             }
//         }
//         maxk
//     }
// }

// Const-ify sol'n
// pub const fn find_max_k_const(nums: &[i32]) -> i32 {
//     let n = nums.len();
//     assert!(n >= 1);
//     assert!(n <= 1000);
//     let mut found = [0u8; 1001];
//     let mut idx = 0;
//     let mut maxk = -1;
//     while idx < n {
//         let num = nums[idx];
//         if num < 0 {
//             let num = -num;
//             found[num as usize] |= 1;
//             if found[num as usize] == 3 {
//                 if num > maxk {
//                     maxk = num;
//                 }
//             }
//         } else {
//             // num > 0
//             found[num as usize] |= 2;
//             if found[num as usize] == 3 {
//                 if num > maxk {
//                     maxk = num;
//                 }
//             }
//         }
//         idx += 1;
//     }
//     maxk
// }
// impl Solution {
//     pub fn find_max_k(nums: Vec<i32>) -> i32 {
//         find_max_k_const(&nums)
//     }
// }

// Try to merge branches
pub const fn find_max_k_const(nums: &[i32]) -> i32 {
    let n = nums.len();
    assert!(n >= 1);
    assert!(n <= 1000);
    let mut found = [0u8; 1001];
    let mut idx = 0;
    let mut maxk = -1;
    while idx < n {
        let num = nums[idx];
        let absnum = if num < 0 { -num } else { num };
        found[absnum as usize] |= if num < 0 { 1 } else { 2 };
        if found[absnum as usize] == 3 {
            if absnum > maxk {
                maxk = absnum;
            }
        }
        idx += 1;
    }
    maxk
}
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        find_max_k_const(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);
        assert!(nums.iter().all(|&n| n <= 1000));
        assert!(nums.iter().all(|&n| n != 0));
        assert!(nums.iter().all(|&n| n >= -1000));
        let result = Solution::find_max_k(nums.to_vec());
        assert!(result > 0 || result == -1);
        assert_eq!(result, expected)
    }

    #[test]
    fn ex1() {
        test(&[-1, 2, -3, 3], 3)
    }

    #[test]
    fn ex2() {
        test(&[-1, 10, 6, 7, -7, 1], 7)
    }

    #[test]
    fn ex3() {
        test(&[-10, 8, 6, 7, -2, -3], -1)
    }
}
