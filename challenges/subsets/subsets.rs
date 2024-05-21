// https://leetcode.com/problems/subsets/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         assert!(nums.len() <= 10); // 2^10 = 1024 (max number of subsets)
//         let mut result: Vec<Vec<i32>> = Vec::with_capacity(1 << nums.len());
//         let mut this_subset: Vec<i32> = Vec::new();
//         fn dfs(nums: &[i32], result: &mut Vec<Vec<i32>>, this_subset: &mut Vec<i32>) {
//             if nums.is_empty() {
//                 result.push(this_subset.clone());
//                 return;
//             }
//             let num = nums[0];
//             let nums = &nums[1..];
//             dfs(&nums, result, this_subset);
//             this_subset.push(num);
//             dfs(&nums, result, this_subset);
//             this_subset.pop();
//         }
//         dfs(&nums, &mut result, &mut this_subset);
//         result
//     }
// }

// Bit select sol'n (too slow)
// impl Solution {
//     pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         assert!(nums.len() <= 10); // 2^10 = 1024 (max number of subsets
//         let cardinality = 1u16 << nums.len();
//         let mut result: Vec<Vec<i32>> = Vec::with_capacity(1 << nums.len());
//         let result_writer = result.spare_capacity_mut();
//         for subset_idx in 0..cardinality {
//             let mut subset: Vec<i32> = Vec::with_capacity(subset_idx.count_ones() as usize);
//             for j in 0..nums.len() as u8 {
//                 if subset_idx & (1 << j) != 0 {
//                     subset.push(nums[j as usize]);
//                 }
//             }
//             result_writer[subset_idx as usize].write(subset);
//         }
//         unsafe { result.set_len(cardinality as usize) };
//         result
//     }
// }

// Revised recursive sol'n
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        assert!(nums.len() <= 10); // 2^10 = 1024 (max number of subsets)
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(1 << nums.len());
        let mut this_subset: Vec<i32> = Vec::new();
        fn dfs(nums: &[i32], result: &mut Vec<Vec<i32>>, this_subset: &mut Vec<i32>, idx: u8) {
            if idx >= nums.len() as u8 {
                result.push(this_subset.clone());
                return;
            }
            let num = nums[idx as usize];
            dfs(&nums, result, this_subset, idx + 1);
            this_subset.push(num);
            dfs(&nums, result, this_subset, idx + 1);
            this_subset.pop();
        }
        dfs(&nums, &mut result, &mut this_subset, 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_powerset(powerset: &mut [Vec<i32>]) {
        for subset in powerset.iter_mut() {
            subset.sort_unstable();
        }
        powerset.sort_unstable();
    }

    fn copy_sort_powerset(powerset: &[&[i32]]) -> Vec<Vec<i32>> {
        let mut powerset_out: Vec<Vec<i32>> =
            powerset.iter().map(|&subset| subset.to_vec()).collect();
        sort_powerset(&mut powerset_out);
        powerset_out
    }

    fn test(input: &[i32], expected: &[&[i32]]) {
        assert!(input.len() > 0);
        assert!(input.len() <= 10);
        assert_eq!(expected.len(), 1 << input.len());
        let expected = copy_sort_powerset(expected);
        let result = {
            let mut result = Solution::subsets(input.to_vec());
            sort_powerset(&mut result);
            result
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(
            &[1, 2, 3],
            &[&[], &[1], &[2], &[1, 2], &[3], &[1, 3], &[2, 3], &[1, 2, 3]],
        );
    }

    #[test]
    fn ex2() {
        test(&[0], &[&[], &[0]]);
    }

    #[test]
    fn myex1() {
        test(&[1], &[&[], &[1]]);
    }

    #[test]
    fn myex2() {
        test(&[1, 2], &[&[], &[1], &[2], &[1, 2]]);
    }
}
