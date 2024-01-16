// https://leetcode.com/problems/3sum/
pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let nums = nums;

        let mut result = Vec::new();

        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // Skip duplicates
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                match (nums[i] + nums[j] + nums[k]).cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;
                        while j < k && nums[j] == nums[j - 1] {
                            j += 1; // Skip duplicates
                        }
                    }
                    std::cmp::Ordering::Less => {
                        j += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        k -= 1;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn do_test(nums: &[i32], expected: &[&[i32]]) {
        let mut expected = expected
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<i32>>>();
        expected.sort_unstable();

        assert_eq!(Solution::three_sum(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        const NUMS: &[i32] = &[-1, 0, 1, 2, -1, -4];
        const EXPECTED: &[&[i32]] = &[&[-1, 0, 1], &[-1, -1, 2]];

        do_test(NUMS, EXPECTED)
    }

    #[test]
    fn ex2() {
        const NUMS: &[i32] = &[0, 1, 1];
        const EXPECTED: &[&[i32]] = &[];

        do_test(NUMS, EXPECTED)
    }

    #[test]
    fn ex3() {
        const NUMS: &[i32] = &[0, 0, 0];
        const EXPECTED: &[&[i32]] = &[&[0, 0, 0]];

        do_test(NUMS, EXPECTED)
    }

    #[test]
    fn myex0() {
        const NUMS: &[i32] = &[-2, 0, 0, 2, 2];
        const EXPECTED: &[&[i32]] = &[&[-2, 0, 2]];

        do_test(NUMS, EXPECTED)
    }

    #[test]
    fn myex1() {
        const NUMS: &[i32] = &[-2, 0, 1, 1, 2];
        const EXPECTED: &[&[i32]] = &[&[-2, 0, 2], &[-2, 1, 1]];

        do_test(NUMS, EXPECTED)
    }

    #[test]
    fn myex2() {
        const NUMS: &[i32] = &[-2, -2, -3, 0, 1, 1, 2];
        const EXPECTED: &[&[i32]] = &[&[-3, 1, 2], &[-2, 0, 2], &[-2, 1, 1]];

        do_test(NUMS, EXPECTED)
    }
}
