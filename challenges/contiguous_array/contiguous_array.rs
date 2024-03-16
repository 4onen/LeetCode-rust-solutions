// https://leetcode.com/problems/contiguous-array/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_max_length(nums: Vec<i32>) -> i32 {
//         assert!(nums.len() >= 1);
//         assert!(nums.len() <= 100_000);
//         let mut sum: i32 = 0;
//         let mut indices = std::collections::HashMap::new();
//         indices.insert(0, -1);
//         let mut max_length = 0;
//         for i in 0..nums.len() as i32 {
//             sum += if nums[i as usize] == 0 { -1 } else { 1 };
//             match indices.entry(sum) {
//                 std::collections::hash_map::Entry::Vacant(e) => {
//                     e.insert(i);
//                 }
//                 std::collections::hash_map::Entry::Occupied(e) => {
//                     max_length = std::cmp::max(max_length, i - e.get());
//                 }
//             }
//         }
//         max_length
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        let mut indices = std::collections::HashMap::new();
        let mut sum: i32 = 0;
        let mut max_length: i32 = 0;
        indices.insert(0, -1);
        for (i, num) in nums.iter().enumerate() {
            sum += if num == &0 { -1 } else { 1 };
            if let Some(&j) = indices.get(&sum) {
                max_length = max_length.max(i as i32 - j);
            } else {
                indices.insert(sum, i as i32);
            }
        }
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![0, 1];
        assert_eq!(Solution::find_max_length(nums), 2);
    }

    #[test]
    fn ex2() {
        let nums = vec![0, 1, 0];
        assert_eq!(Solution::find_max_length(nums), 2);
    }

    #[test]
    fn my_extreme_example1() {
        let nums = vec![1; 100_000];
        assert_eq!(Solution::find_max_length(nums), 0);
    }

    #[test]
    fn my_extreme_example2() {
        let nums = vec![0; 100_000];
        assert_eq!(Solution::find_max_length(nums), 0);
    }

    #[test]
    fn my_extreme_example3() {
        let mut nums = vec![1; 50_000];
        nums.extend([0; 50_000]);
        assert_eq!(Solution::find_max_length(nums), 100_000);
    }
}
