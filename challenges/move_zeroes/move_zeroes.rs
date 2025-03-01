// https://leetcode.com/problems/move-zeroes/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn move_zeroes(nums: &mut Vec<i32>) {
//         let mut read = 0;
//         let mut write = 0;
//         while read < nums.len() {
//             if nums[read] != 0 {
//                 nums[write] = nums[read];
//                 write += 1;
//             }
//             read += 1;
//         }
//         while write < nums.len() {
//             nums[write] = 0;
//             write += 1;
//         }
//     }
// }

// .fill(0) and u16 len
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut read: u16 = 0;
        let mut write: u16 = 0;
        while read < nums.len() as u16 {
            if nums[read as usize] != 0 {
                nums[write as usize] = nums[read as usize];
                write += 1;
            }
            read += 1;
        }
        nums[write as usize..].fill(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: &[i32]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 10_000);
        assert_eq!(nums.len(), expected.len());
        {
            // Check nonzero elements are in the same order
            let mut i = 0;
            let mut j = 0;
            while i < nums.len() && j < expected.len() {
                if nums[i] == 0 {
                    i += 1;
                    continue;
                }
                if expected[j] == 0 {
                    j += 1;
                    continue;
                }
                assert_eq!(nums[i], expected[j]);
                i += 1;
                j += 1;
            }
        }
        {
            // Check the expected array only has zeros at the end
            let mut first_zero_seen = false;
            for &x in expected {
                if first_zero_seen {
                    assert_eq!(x, 0);
                }
                first_zero_seen |= x == 0;
            }
        }
        let mut result = nums.to_vec();
        Solution::move_zeroes(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 0, 3, 12], &[1, 3, 12, 0, 0])
    }

    #[test]
    fn ex2() {
        test(&[0], &[0])
    }
}
