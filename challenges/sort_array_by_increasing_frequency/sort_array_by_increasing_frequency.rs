// https://leetcode.com/problems/sort-array-by-increasing-frequency/

pub struct Solution;

// Initial sol'n (bad indexer)
// impl Solution {
//     pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
//         let mut counts = [0u8; 256];
//         for &num in nums.iter() {
//             counts[(num as i8 & i8::MAX) as usize] += 1;
//         }
//         nums.sort_unstable_by(|&a, &b| {
//             let count_a = counts[(a as i8 & i8::MAX) as usize];
//             let count_b = counts[(b as i8 & i8::MAX) as usize];
//             count_a.cmp(&count_b).then_with(|| b.cmp(&a))
//         });
//         nums
//     }
// }

// Fixed indexer
impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0u8; 201];
        for &num in nums.iter() {
            counts[(num as u8).wrapping_add(100) as usize] += 1;
        }
        nums.sort_unstable_by(|&a, &b| {
            let count_a = counts[(a as u8).wrapping_add(100) as usize];
            let count_b = counts[(b as u8).wrapping_add(100) as usize];
            count_a.cmp(&count_b).then_with(|| b.cmp(&a))
        });
        nums
    }
}

// Use sort_unstable_by_key instead
// (Untested on LeetCode 'cause they just went down.)
// impl Solution {
//     pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
//         let mut counts = [0u8; 201];
//         for &num in nums.iter() {
//             counts[(num as u8).wrapping_add(100) as usize] += 1;
//         }
//         nums.sort_unstable_by_key(|&num| (counts[(num as u8).wrapping_add(100) as usize], -(num as i8)));
//         nums
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i8], expected: &[i8]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= -100);
            assert!(num <= 100);
            //dbg!(num, num & i8::MAX);
            dbg!(num, (num as u8).wrapping_add(100));
        }
        assert_eq!(expected.len(), nums.len());
        let nums = nums.iter().map(|&num| num as i32).collect::<Vec<_>>();
        let result = Solution::frequency_sort(nums.to_vec());
        let result_bytes = result
            .iter()
            .map(|&num| {
                assert!(num >= -100);
                assert!(num <= 100);
                num as i8
            })
            .collect::<Vec<_>>();
        assert_eq!(result_bytes, expected)
    }

    #[test]
    fn ex1() {
        test(&[1, 1, 2, 2, 2, 3], &[3, 1, 1, 2, 2, 2])
    }

    #[test]
    fn ex2() {
        test(&[2, 3, 1, 3, 2], &[1, 3, 3, 2, 2])
    }

    #[test]
    fn ex3() {
        test(
            &[-1, 1, -6, 4, 5, -6, 1, 4, 1],
            &[5, -1, 4, 4, -6, -6, 1, 1, 1],
        )
    }

    #[test]
    fn failing_case1() {
        test(
            &[
                -79, 40, 21, 40, 30, -100, -50, -79, -51, 30, -65, -13, -46, 100, 40, -65, -13,
                100, 40, -79, 55, 68, 55, 68, 68, 30, 79, -51, 68, 21, -60, 40, 79, 30, 55, -65,
                -13, -46, -100, -50, 21, -60, -51, 100, -51, -50, 30, 100, 40, 68, -13, -65,
            ],
            &[
                79, 79, -46, -46, -60, -60, -100, -100, 55, 55, 55, 21, 21, 21, -50, -50, -50, -79,
                -79, -79, 100, 100, 100, 100, -13, -13, -13, -13, -51, -51, -51, -51, -65, -65,
                -65, -65, 68, 68, 68, 68, 68, 30, 30, 30, 30, 30, 40, 40, 40, 40, 40, 40,
            ],
        )
    }
}
