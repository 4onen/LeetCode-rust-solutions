pub struct Solution;

impl Solution {
    // Mutates array in place to remove duplicates, leaving garbage past the end
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // First element is never a duplicate, for obvious reasons
        let mut out_idx = 1;
        for in_idx in 1..nums.len() {
            if nums[in_idx] != nums[out_idx - 1] {
                nums[out_idx] = nums[in_idx];
                out_idx += 1;
            }
        }
        out_idx as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
    }
}
