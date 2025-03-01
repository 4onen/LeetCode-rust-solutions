// https://leetcode.com/problems/apply-operations-to-an-array/

pub struct Solution;

// Initial inplace sol'n
// impl Solution {
//     pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
//         let mut i = 0;
//         let mut write = 0;
//         while i < nums.len() {
//             if nums[i] == 0 {
//                 i += 1;
//             } else if i < nums.len() - 1 && nums[i] == nums[i + 1] {
//                 nums[write] = nums[i] * 2;
//                 nums[i + 1] = 0;
//                 write += 1;
//                 i += 2;
//             } else {
//                 nums[write] = nums[i];
//                 i += 1;
//                 write += 1;
//             }
//         }
//         nums[write..].fill(0);
//         nums
//     }
// }

// Const inplace sol'n (Not yet possible in LeetCode's Rust ver)
impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        const fn apply_operations_inplace(nums: &mut [i32]) {
            let mut i = 0;
            let mut write = 0;
            while i < nums.len() {
                if nums[i] == 0 {
                    i += 1;
                } else if i < nums.len() - 1 && nums[i] == nums[i + 1] {
                    nums[write] = nums[i] * 2;
                    nums[i + 1] = 0;
                    write += 1;
                    i += 2;
                } else {
                    nums[write] = nums[i];
                    i += 1;
                    write += 1;
                }
            }
            while write < nums.len() {
                nums[write] = 0;
                write += 1;
            }
        }
        apply_operations_inplace(&mut nums);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u16], expected: &[i32]) {
        assert!(nums.len() >= 2);
        assert!(nums.len() <= 2000);
        for &num in nums {
            assert!(num <= 1000);
        }
        assert_eq!(nums.len(), expected.len());
        assert_eq!(
            Solution::apply_operations(nums.iter().map(|&x| x as i32).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 2, 1, 1, 0], &[1, 4, 2, 0, 0, 0])
    }

    #[test]
    fn ex2() {
        test(&[0, 1], &[1, 0])
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                847, 847, 0, 0, 0, 399, 416, 416, 879, 879, 206, 206, 206, 272,
            ],
            &[1694, 399, 832, 1758, 412, 206, 272, 0, 0, 0, 0, 0, 0, 0],
        )
    }
}
