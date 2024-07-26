// https://leetcode.com/problems/sort-an-array/

pub struct Solution;

// Quick mergesort with insertion sort for small arrays
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        // fn my_merge_impl(mid: u16, nums: &mut [i32]) {
        //     // In-place merge
        //     let mut i = 0u16;
        //     let mut j = mid - 1;
        //     let mut k = mid;
        //     if nums[j as usize] > nums[k as usize] {
        //         while i <= j && k < nums.len() as u16 {
        //             if nums[i as usize] <= nums[k as usize] {
        //                 i += 1;
        //             } else {
        //                 let value = nums[k as usize];
        //                 let mut index = k;
        //                 while index > i {
        //                     nums[index as usize] = nums[(index - 1) as usize];
        //                     index -= 1;
        //                 }
        //                 nums[i as usize] = value;
        //                 i += 1;
        //                 j += 1;
        //                 k += 1;
        //             }
        //         }
        //     } // Else already sorted
        // }
        fn my_merge_impl(mid: u16, nums: &mut [i32]) {
            // In-place merge with lib rotate (10-20x faster)
            let mut i = 0u16;
            let mut j = mid-1;
            let mut k = mid;
            if nums[j as usize] > nums[k as usize] {
                while i <= j && k < nums.len() as u16 {
                    if nums[i as usize] <= nums[k as usize] {
                        i += 1;
                    } else {
                        nums[i as usize..=k as usize].rotate_right(1);
                        i += 1;
                        j += 1;
                        k += 1;
                    }
                }
            } // Else already sorted
        }
        // fn my_merge_impl(mid: u16, nums: &mut [i32]) {
        //     // Out-of-place merge
        //     let to_merge = nums.to_vec();
        //     let mut i = 0u16;
        //     let mut j = mid;
        //     while i < j && j < to_merge.len() as u16 {
        //         let k = i + j - mid;
        //         if to_merge[i as usize] <= to_merge[j as usize] {
        //             nums[k as usize] = to_merge[i as usize];
        //             i += 1;
        //         } else {
        //             nums[k as usize] = to_merge[j as usize];
        //             j += 1;
        //         }
        //     }
        //     let k = i + j - mid;
        //     if j < to_merge.len() as u16 {
        //         nums[k as usize..].copy_from_slice(&to_merge[j as usize..]);
        //     } else if i < j {
        //         nums[k as usize..].copy_from_slice(&to_merge[i as usize..j as usize]);
        //     }
        // }
        fn my_sort_impl(nums: &mut [i32]) {
            match nums.len() {
                0 | 1 => {}
                2 => {
                    if nums[0] > nums[1] {
                        nums.swap(0, 1);
                    }
                }
                n @ (3..=200) => {
                    for i in 1..n {
                        let mut j = i;
                        while j > 0 && nums[j] < nums[j - 1] {
                            nums.swap(j, j - 1);
                            j -= 1;
                        }
                    }
                }
                n @ (201..=50_000) => {
                    let n = n as u16;
                    let mid = n / 2;
                    my_sort_impl(&mut nums[..mid as usize]);
                    my_sort_impl(&mut nums[mid as usize..]);
                    my_merge_impl(mid, nums);
                }
                50_001.. => unreachable!("Limit exceeded"),
            }
        }
        my_sort_impl(&mut nums);
        nums
    }
}

// Quicksort with insertion sort for small arrays
// (Twice as slow as the above on LeetCode because of non-optimal pivot)
// impl Solution {
//     pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
//         fn my_partition(nums: &mut [i32]) -> u16 {
//             let pivot = nums[nums.len() / 2];
//             let mut i = 0u16;
//             let mut j = nums.len() as u16 - 1;
//             loop {
//                 while nums[i as usize] < pivot {
//                     i += 1;
//                 }
//                 while nums[j as usize] > pivot {
//                     j -= 1;
//                 }
//                 if i >= j {
//                     return j;
//                 }
//                 nums.swap(i as usize, j as usize);
//                 i += 1;
//                 j -= 1;
//             }
//         }
//         fn my_sort_impl(nums: &mut [i32]) {
//             match nums.len() {
//                 0 | 1 => {}
//                 2 => {
//                     if nums[0] > nums[1] {
//                         nums.swap(0, 1);
//                     }
//                 }
//                 n @ (3..=200) => {
//                     for i in 1..n {
//                         let mut j = i;
//                         while j > 0 && nums[j] < nums[j - 1] {
//                             nums.swap(j, j - 1);
//                             j -= 1;
//                         }
//                     }
//                 }
//                 201..=50_000 => {
//                     let mid = my_partition(nums);
//                     my_sort_impl(&mut nums[..=mid as usize]);
//                     my_sort_impl(&mut nums[mid  as usize+ 1..]);
//                 }
//                 50_001.. => unreachable!("Limit exceeded"),
//             }
//         }
//         my_sort_impl(&mut nums);
//         nums
//     }
// }

// Quicksort with insertion and counting sort for small arrays
// (Broken somewhere, but I can't find it)
// impl Solution {
//     pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
//         type CountingVec = Vec<u16>;
//         fn my_partition(nums: &mut [i32]) -> u16 {
//             let pivot = nums[nums.len() / 2];
//             let mut i = 0u16;
//             let mut j = nums.len() as u16 - 1;
//             loop {
//                 while nums[i as usize] < pivot {
//                     i += 1;
//                 }
//                 while nums[j as usize] > pivot {
//                     j -= 1;
//                 }
//                 if i >= j {
//                     break i;
//                 }
//                 nums.swap(i as usize, j as usize);
//                 i += 1;
//                 j -= 1;
//             }
//         }
//         fn my_sort_impl(nums: &mut [i32], min: i32, max: i32, counting_vec: &mut Option<CountingVec>, original_length: i32) {
//             match (nums.len(), (max - min + 1) < original_length) {
//                 (0,_) | (1,_) => {}
//                 (n @ (2..=200), _) => {
//                     for i in 1..n {
//                         let mut j = i;
//                         while j > 0 && nums[j] < nums[j - 1] {
//                             nums.swap(j, j - 1);
//                             j -= 1;
//                         }
//                     }
//                 }
//                 (201..=50_000, true) => {
//                     let counting_vec = counting_vec.get_or_insert_with(|| Vec::with_capacity(original_length as usize));
//                     counting_vec.truncate(0);
//                     let counting_vec_space = counting_vec.spare_capacity_mut();
//                     counting_vec_space[..(max - min + 1) as usize].iter_mut().for_each(|x| {x.write(0);});
//                     unsafe {
//                         counting_vec.set_len((max - min + 1) as usize);
//                     }
//                     for &num in nums.iter() {
//                         counting_vec[(num - min) as usize] += 1;
//                     }
//                     let mut index = 0;
//                     for (num, &count) in counting_vec.iter().enumerate() {
//                         for _ in 0..count {
//                             nums[index] = num as i32 + min;
//                             index += 1;
//                         }
//                     }
//                 }
//                 (201..=50_000, false) => {
//                     let pivot_point = my_partition(nums);
//                     let pivot_value = nums[pivot_point as usize];
//                     let left_slice = &mut nums[..pivot_point as usize];
//                     let (dbg_min, dbg_max) = min_max(left_slice);
//                     assert!(dbg_min <= pivot_value, "min: {}, pivot: {}", dbg_min, pivot_value);
//                     assert!(dbg_max <= pivot_value, "max: {}, pivot: {}", dbg_max, pivot_value);
//                     my_sort_impl(left_slice, min, pivot_value, counting_vec, original_length);
//                     my_sort_impl(&mut nums[pivot_point as usize + 1..], pivot_value, max, counting_vec, original_length);
//                 }
//                 (50_001.., _) => unreachable!("Limit exceeded"),
//             }
//         }
//         fn min_max(nums: &[i32]) -> (i32, i32) {
//             let mut min = i32::MAX;
//             let mut max = i32::MIN;
//             for &num in nums.iter() {
//                 if num < min {
//                     min = num;
//                 }
//                 if num > max {
//                     max = num;
//                 }
//             }
//             (min, max)
//         }
//         let mut counting_vec: Option<CountingVec> = None;
//         let (min,max) = min_max(&nums);
//         let original_length = nums.len() as i32;
//         my_sort_impl(&mut nums, min, max, &mut counting_vec, original_length);
//         nums
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32]) {
        let mut sorted = nums.to_vec();
        sorted.sort_unstable();
        assert_eq!(Solution::sort_array(nums.to_vec()), sorted)
    }

    #[test]
    fn ex1() {
        test(&[5, 2, 3, 1])
    }

    #[test]
    fn myex1_1() {
        test(&[3, 1, 2])
    }

    #[test]
    fn ex2() {
        test(&[5, 1, 1, 2, 0, 0])
    }

    #[test]
    fn myex3() {
        test(&[1, 2, 3, 4, 5])
    }

    #[test]
    fn my_extreme_ex1() {
        // 50_000 elements all at 50_000
        // The numeric limits of the problem.
        test(&[50_000; 50_000])
    }

    #[test]
    fn my_extreme_ex2() {
        // 50_000 elements all at -50_000
        // The numeric limits of the problem.
        test(&[-50_000; 50_000])
    }

    #[test]
    fn my_extreme_ex3() {
        // 50_000 elements all at 0
        test(&[0; 50_000])
    }

    #[test]
    fn my_extreme_ex4() {
        // 50_000 elements all at 0, except the last one at 1
        test(
            &[0; 49_999]
                .iter()
                .copied()
                .chain(std::iter::once(1))
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn my_extreme_ex5() {
        // 50_000 elements all at 0, except the first one at -1
        test(
            &[-1]
                .iter()
                .copied()
                .chain(std::iter::repeat(0).take(49_999))
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn my_extreme_ex6() {
        // 50_000 elements all at 0, except the middle one at 1
        test(
            &[0; 25_000]
                .iter()
                .copied()
                .chain(std::iter::once(1))
                .chain(std::iter::repeat(0).take(24_999))
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn myex6() {
        test(
            &[0; 50]
                .iter()
                .copied()
                .chain(std::iter::once(1).chain(std::iter::repeat(0).take(50)))
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn tle_case1() {
        let input_str = include_str!("tle_case1.txt");
        // Need to trim off a [ and ] at the beginning and end of the string
        let input: Vec<i32> = input_str
            .trim_start_matches('[')
            .trim_end_matches("]\n")
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        test(&input)
    }

    #[test]
    fn my_extreme_reversed_ex1() {
        // 50_000 elements counting down from 50_000
        test(&(1..=50_000).rev().collect::<Vec<_>>())
    }

    #[test]
    fn my_extreme_reversed_ex2() {
        // 50_000 elements counting down from 0
        test(&(1..=50_000).rev().map(|x| x - 50_000).collect::<Vec<_>>())
    }

    #[test]
    fn discussion_case1() {
        let input_str = include_str!("discussion_case1.txt");
        // Need to trim off a [ and ] at the beginning and end of the string
        let input: Vec<i32> = input_str
            .trim_start_matches('[')
            .trim_end_matches('\n')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        test(&input)
    }
}
