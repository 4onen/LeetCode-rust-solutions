// https://leetcode.com/problems/partition-array-according-to-given-pivot/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
//         let mut lefts = 0;
//         let mut rights = 0;
//         for &num in &nums {
//             if num < pivot {
//                 lefts += 1;
//             } else if num > pivot {
//                 rights += 1;
//             }
//         }
//         let mut result = vec![0; nums.len()];
//         let mut left_write = 0;
//         let mut right_write = nums.len() - rights;
//         result[lefts..right_write].fill(pivot);
//         for &num in &nums {
//             if num < pivot {
//                 result[left_write] = num;
//                 left_write += 1;
//             } else if num > pivot {
//                 result[right_write] = num;
//                 right_write += 1;
//             }
//         }
//         result
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut rights = std::vec::Vec::with_capacity(nums.len());
        let mut pivots = 0;
        let mut left_write = 0;
        for i in 0..nums.len() {
            let num = nums[i];
            if num > pivot {
                rights.push(num);
            } else if num == pivot {
                pivots += 1;
            } else {
                nums[left_write] = num;
                left_write += 1;
            }
        }
        nums.truncate(left_write + pivots);
        nums[left_write..].fill(pivot);
        nums.extend_from_slice(&rights);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], pivot: i32, expected: &[i32]) {
        assert_eq!(Solution::pivot_array(nums.to_vec(), pivot as i32), expected);
    }

    #[test]
    fn ex1() {
        test(&[9, 12, 5, 10, 14, 3, 10], 10, &[9, 5, 3, 10, 10, 12, 14])
    }

    #[test]
    fn ex2() {
        test(&[-3, 4, 3, 2], 2, &[-3, 2, 4, 3])
    }
}
