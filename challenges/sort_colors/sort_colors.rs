// https://leetcode.com/problems/sort-colors/

pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut zero_ptr = 0;
        let mut two_ptr = nums.len() - 1;
        let mut read_ptr = 0;
        while read_ptr <= two_ptr {
            if nums[read_ptr] == 0 {
                nums.swap(read_ptr, zero_ptr);
                zero_ptr += 1;
                read_ptr += 1;
            } else if nums[read_ptr] == 2 {
                nums.swap(read_ptr, two_ptr);
                if two_ptr == 0 {
                    break;
                }
                two_ptr -= 1;
            } else {
                read_ptr += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: &[i32]) {
        assert_eq!(nums.len(), expected.len());
        assert!(nums.len() > 0);
        assert!(nums.len() <= 300);
        for &n in nums {
            assert!(n >= 0);
            assert!(n <= 2);
        }
        let mut nums = nums.to_vec();
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 0, 2, 1, 1, 0], &[0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn ex2() {
        test(&[2, 0, 1], &[0, 1, 2]);
    }

    #[test]
    fn myex0() {
        test(&[0], &[0]);
    }

    #[test]
    fn myex1() {
        test(&[1, 0], &[0, 1]);
    }

    #[test]
    fn myex2() {
        test(&[1, 2, 0], &[0, 1, 2]);
    }

    #[test]
    fn myex3() {
        test(&[1, 2, 0, 1, 2, 0], &[0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn myex4() {
        test(&[1, 2, 0, 1, 2, 0, 1, 2, 0], &[0, 0, 0, 1, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn myex300_0() {
        let nums = [0; 300];
        test(&nums, &nums)
    }

    #[test]
    fn myex300_1() {
        let nums = [1; 300];
        test(&nums, &nums)
    }

    #[test]
    fn myex300_2() {
        let nums = [2; 300];
        test(&nums, &nums)
    }

    #[test]
    fn myex300_012() {
        let zeros = [0; 100];
        let ones = [1; 100];
        let twos = [2; 100];

        let mut nums = Vec::with_capacity(zeros.len() + ones.len() + twos.len());
        nums.extend_from_slice(&zeros);
        nums.extend_from_slice(&ones);
        nums.extend_from_slice(&twos);

        test(&nums, &nums)
    }

    #[test]
    fn myex300_012_2() {
        let zeros = [0; 100];
        let ones = [1; 100];
        let twos = [2; 100];

        let mut nums = Vec::with_capacity(zeros.len() + ones.len() + twos.len());
        nums.extend_from_slice(&twos);
        nums.extend_from_slice(&zeros);
        nums.extend_from_slice(&ones);

        let mut expected = Vec::with_capacity(zeros.len() + ones.len() + twos.len());
        expected.extend_from_slice(&zeros);
        expected.extend_from_slice(&ones);
        expected.extend_from_slice(&twos);

        test(&nums, &expected)
    }

    #[test]
    fn myex300_012_3() {
        let zeros = [0; 100];
        let ones = [1; 100];
        let twos = [2; 100];

        let mut nums = Vec::with_capacity(zeros.len() + ones.len() + twos.len());
        nums.extend_from_slice(&ones);
        nums.extend_from_slice(&twos);
        nums.extend_from_slice(&zeros);

        let mut expected = Vec::with_capacity(zeros.len() + ones.len() + twos.len());
        expected.extend_from_slice(&zeros);
        expected.extend_from_slice(&ones);
        expected.extend_from_slice(&twos);

        test(&nums, &expected)
    }

    #[test]
    fn myex300_012_4() {
        let zeros = [0; 100];
        let ones = [1; 100];
        let twos = [2; 100];

        let mut nums = Vec::with_capacity(zeros.len() + ones.len() + twos.len());
        nums.extend_from_slice(&ones);
        nums.extend_from_slice(&zeros);
        nums.extend_from_slice(&twos);

        let mut expected = Vec::with_capacity(zeros.len() + ones.len() + twos.len());
        expected.extend_from_slice(&zeros);
        expected.extend_from_slice(&ones);
        expected.extend_from_slice(&twos);

        test(&nums, &expected)
    }
}
