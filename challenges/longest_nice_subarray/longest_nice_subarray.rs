// https://leetcode.com/problems/longest-nice-subarray/

pub struct Solution;

// Naive sol'n
// impl Solution {
//     pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
//         let mut best = 1;
//         for i in 0..nums.len() {
//             for j in i + 1..nums.len() {
//                 if nums[i] & nums[j] == 0 {
//                     let len = j - i + 1;
//                     if len > best {
//                         best = len;
//                     }
//                 } else {
//                     break;
//                 }
//             }
//         }
//         best as i32
//     }
// }

// Sliding window sol'n
// impl Solution {
//     pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
//         let nums_len = nums.len() as u32;
//         let mut best = 1;
//         let mut i = 0u32;
//         let mut j = 1u32;
//         let mut acc = nums[0];
//         while j < nums_len {
//             while i < j && acc & nums[j as usize] != 0 {
//                 acc ^= nums[i as usize];
//                 i += 1;
//             }
//             acc |= nums[j as usize];
//             let len = j - i + 1;
//             if len > best {
//                 best = len;
//             }
//             j += 1;
//         }
//         best as i32
//     }
// }

// Optimized sliding window sol'n
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len() as u32;
        let mut best = 1;
        let mut i = 0u32;
        let mut acc = nums[0];
        for j in 1..nums_len {
            while i < j && acc & nums[j as usize] != 0 {
                acc ^= nums[i as usize];
                i += 1;
            }
            acc |= nums[j as usize];
            let len = j - i + 1;
            if len > best {
                best = len;
            }
        }
        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 1_000_000_000);
        }
        assert_eq!(Solution::longest_nice_subarray(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 3, 8, 48, 10], 3)
    }

    #[test]
    fn ex2() {
        test(&[3, 1, 5, 11, 13], 1)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                84139415, 693324769, 614626365, 497710833, 615598711, 264, 65552, 50331652, 1,
                1048576, 16384, 544, 270532608, 151813349, 221976871, 678178917, 845710321,
                751376227, 331656525, 739558112, 267703680,
            ],
            8,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                178830999, 19325904, 844110858, 806734874, 280746028, 64, 256, 33554432, 882197187,
                104359873, 453049214, 820924081, 624788281, 710612132, 839991691,
            ],
            4,
        )
    }
}
