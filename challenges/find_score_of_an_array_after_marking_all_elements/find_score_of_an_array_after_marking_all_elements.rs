// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_score(mut nums: Vec<i32>) -> i64 {
//         let mut elements: std::collections::BinaryHeap<_> = nums
//             .iter()
//             .copied()
//             .enumerate()
//             .map(|(i, num)| std::cmp::Reverse((num, i as u32)))
//             .collect();
//         let mut score: i64 = 0;
//         loop {
//             let Some(std::cmp::Reverse((num, i))) = elements.pop() else {
//                 break;
//             };
//             if nums[i as usize] > 0 {
//                 score += num as i64;
//                 nums[i as usize] = 0;
//                 if i > 0 {
//                     nums[i as usize - 1] = 0;
//                 }
//                 if i as usize + 1 < nums.len() {
//                     nums[i as usize + 1] = 0;
//                 }
//             }
//         }
//         score
//     }
// }

// Sorting sol'n
// impl Solution {
//     pub fn find_score(mut nums: Vec<i32>) -> i64 {
//         let mut elements = nums
//             .iter()
//             .copied()
//             .enumerate()
//             .map(|(i, num)| (i as u32, num))
//             .collect::<Vec<_>>();
//         elements.sort_unstable_by_key(|&(i, num)| (num, i));
//         let mut score: i64 = 0;
//         for (i, num) in elements {
//             if nums[i as usize] > 0 {
//                 score += num as i64;
//                 nums[i as usize] = 0;
//                 if i > 0 {
//                     nums[i as usize - 1] = 0;
//                 }
//                 if i as usize + 1 < nums.len() {
//                     nums[i as usize + 1] = 0;
//                 }
//             }
//         }
//         score
//     }
// }

// Sorting indices only (slower)
// impl Solution {
//     pub fn find_score(mut nums: Vec<i32>) -> i64 {
//         let mut indices = (0..nums.len()).collect::<Vec<_>>();
//         indices.sort_unstable_by_key(|&i| (nums[i], i));
//         let mut score: i64 = 0;
//         for i in indices {
//             if nums[i] > 0 {
//                 score += nums[i] as i64;
//                 nums[i] = 0;
//                 if i > 0 {
//                     nums[i - 1] = 0;
//                 }
//                 if i + 1 < nums.len() {
//                     nums[i + 1] = 0;
//                 }
//             }
//         }
//         score
//     }
// }

// Sorting indices only, remembering to compress their dtype (still slower)
// impl Solution {
//     pub fn find_score(mut nums: Vec<i32>) -> i64 {
//         let mut indices = (0..nums.len() as u32).collect::<Vec<_>>();
//         indices.sort_unstable_by_key(|&i| (nums[i as usize], i));
//         let mut score: i64 = 0;
//         for i in indices {
//             if nums[i as usize] > 0 {
//                 score += nums[i as usize] as i64;
//                 nums[i as usize] = 0;
//                 if i > 0 {
//                     nums[i as usize - 1] = 0;
//                 }
//                 if i + 1 < nums.len() as u32 {
//                     nums[i as usize + 1] = 0;
//                 }
//             }
//         }
//         score
//     }
// }

// Sorting sol'n with reduced marking steps
// impl Solution {
//     pub fn find_score(mut nums: Vec<i32>) -> i64 {
//         let mut elements = nums
//             .iter()
//             .copied()
//             .enumerate()
//             .map(|(i, num)| (i as u32, num))
//             .collect::<Vec<_>>();
//         elements.sort_unstable_by_key(|&(i, num)| (num, i));
//         let mut score: i64 = 0;
//         for (i, num) in elements {
//             if nums[i as usize] > 0 {
//                 score += num as i64;
//                 if i > 0 {
//                     nums[i as usize - 1] = 0;
//                 }
//                 if i as usize + 1 < nums.len() {
//                     nums[i as usize + 1] = 0;
//                 }
//             }
//         }
//         score
//     }
// }

// Sorting sol'n with bitset marking list
// 128-bit bitset: 16ms
// 64-bit bitset: 12ms
// 32-bit bitset: 11ms
// 16-bit bitset: 15ms
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let elements = {
            let mut elements = nums
                .into_iter()
                .enumerate()
                .map(|(i, num)| (num, i as u32))
                .collect::<Vec<_>>();
            elements.sort_unstable();
            elements
        };
        type BitSetDtype = u32;
        const BITS: u32 = BitSetDtype::BITS;
        let mut seen: Vec<BitSetDtype> = vec![0; ((elements.len() as u32 + 1) / BITS + 1) as usize];
        elements.into_iter().fold(0i64, |score, (num, i)| {
            if (seen[(i / BITS) as usize] >> (i % BITS)) & 1 == 1 {
                score
            } else {
                if i > 0 {
                    seen[((i - 1) / BITS) as usize] |= 1 << ((i - 1) % BITS);
                }
                seen[((i + 1) / BITS) as usize] |= 1 << ((i + 1) % BITS);
                score + num as i64
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i64) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 1_000_000);
        }
        assert_eq!(Solution::find_score(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 1, 3, 4, 5, 2], 7)
    }

    #[test]
    fn ex2() {
        test(&[2, 3, 5, 1, 3, 2], 5)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                10, 44, 10, 8, 48, 30, 17, 38, 41, 27, 16, 33, 45, 45, 34, 30, 22, 3, 42, 42,
            ],
            212,
        )
    }
}
