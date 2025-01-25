// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/

pub struct Solution;

// Naive sol'n
// impl Solution {
//     pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
//         'outer: loop {
//             for i in 0..nums.len() as u32 {
//                 for j in i + 1..nums.len() as u32 {
//                     if nums[i as usize] > nums[j as usize]
//                         && nums[i as usize] - nums[j as usize] <= limit
//                     {
//                         nums.swap(i as usize, j as usize);
//                         continue 'outer;
//                     }
//                 }
//             }
//             break nums;
//         }
//     }
// }

// Initial sol'n (untested, way too many allocs)
// impl Solution {
//     pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
//         let mut index_arr: Vec<_> = (0..nums.len()).into_iter().collect();
//         index_arr.sort_unstable_by_key(|&i| nums[i]);
//         let mut group_start = 0;
//         while group_start < index_arr.len() {
//             let mut group_end = group_start + 1;
//             while group_end < nums.len()
//                 && nums[index_arr[group_end]] - nums[index_arr[group_end - 1]] <= limit
//             {
//                 group_end += 1;
//             }
//             let mut sorted_indices = index_arr[group_start..group_end].to_vec();
//             sorted_indices.sort_unstable();
//             let sorted_nums: Vec<_> = index_arr[group_start..group_end]
//                 .iter()
//                 .map(|&i| nums[i])
//                 .collect();
//             for (i, x) in std::iter::zip(sorted_indices.into_iter(), sorted_nums.into_iter()) {
//                 nums[i] = x;
//             }
//             group_start = group_end;
//         }
//         nums
//     }
// }

// Some alloc reuse
impl Solution {
    pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut index_arr: Vec<_> = (0..nums.len()).into_iter().collect();
        index_arr.sort_unstable_by_key(|&i| nums[i]);
        let mut indices_work_vec = vec![];
        let mut nums_work_vec = vec![];
        let mut group_start = 0;
        while group_start < index_arr.len() {
            let mut group_end = group_start + 1;
            while group_end < nums.len()
                && nums[index_arr[group_end]] - nums[index_arr[group_end - 1]] <= limit
            {
                group_end += 1;
            }
            indices_work_vec.extend_from_slice(&index_arr[group_start..group_end].to_vec());
            nums_work_vec.extend(indices_work_vec.iter().map(|&i| nums[i]));
            indices_work_vec.sort_unstable();
            for (i, x) in std::iter::zip(indices_work_vec.drain(..), nums_work_vec.drain(..)) {
                nums[i] = x;
            }
            group_start = group_end;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], limit: i32, expected: &[i32]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        assert!(limit >= 1);
        assert!(limit <= 1_000_000_000);
        for &n in nums {
            assert!(n >= 1);
            assert!(n <= 1_000_000_000);
        }
        assert_eq!(
            Solution::lexicographically_smallest_array(nums.to_vec(), limit),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 5, 3, 9, 8], 2, &[1, 3, 5, 8, 9])
    }

    #[test]
    fn ex2() {
        test(&[1, 7, 6, 18, 2, 1], 3, &[1, 6, 7, 18, 1, 2])
    }

    #[test]
    fn ex3() {
        test(&[1, 7, 28, 19, 10], 3, &[1, 7, 28, 19, 10])
    }

    #[test]
    fn discussion_case1() {
        test(&[5, 5, 5, 6], 3, &[5, 5, 5, 6])
    }

    #[test]
    fn discussion_case2() {
        test(&[1_000_000_000], 1, &[1_000_000_000])
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 7, 6, 18, 2, 1], 4, &[1, 1, 2, 18, 6, 7])
    }

    #[test]
    fn discussion_case4() {
        test(
            &[5, 100, 44, 45, 16, 30, 14, 65, 83, 64],
            15,
            &[5, 100, 14, 16, 30, 44, 45, 64, 83, 65],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        const CNT: usize = 100_000;
        test(&[1_000_000_000; CNT], 1, &[1_000_000_000; CNT])
    }

    #[test]
    fn my_extreme_ex2() {
        const CNT: usize = 100_000;
        let mut input = [0; CNT];
        for i in 0..input.len() {
            input[i] = 1_000_000_000 - i as i32;
        }
        let mut res = input.clone();
        res.sort_unstable();
        test(&input, 1, &res)
    }

    #[test]
    fn my_extreme_ex3() {
        const CNT: usize = 100_000;
        let mut input = [0; CNT];
        for i in 0..input.len() / 2 {
            input[i] = 1_000_000_000 - i as i32;
        }
        for i in input.len() / 2..input.len() {
            input[i] = 1_000_000_000 - i as i32 - 1;
        }
        let mut res = input.clone();
        res[0..input.len() / 2].sort_unstable();
        res[input.len() / 2..input.len()].sort_unstable();
        test(&input, 1, &res)
    }
}
