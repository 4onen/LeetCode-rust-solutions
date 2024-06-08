// https://leetcode.com/problems/continuous-subarray-sum/

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        assert!(k > 0);
        assert!(nums.len() <= 100_000);
        let mut mod_space = std::collections::HashMap::new();
        let mut sum = 0;
        mod_space.insert(0, -1i32);
        for (i, &num) in nums.iter().enumerate() {
            let i = i as i32;
            if num == 0 {
                if i > 0 && nums[i as usize - 1] == 0 {
                    return true;
                }
                continue;
            }
            sum += num;
            sum %= k;
            match mod_space.get(&sum) {
                Some(&j) if i - j > 1 => return true,
                Some(_) => {}
                None => {
                    mod_space.insert(sum, i);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u32], k: u32, expected: bool) {
        assert!(nums.len() > 0);
        assert!(nums.len() <= 100_000);
        assert!(k > 0);
        assert!(k <= i32::MAX as u32);
        for &num in nums {
            assert!(num <= 1_000_000_000);
        }
        assert!(nums.iter().copied().sum::<u32>() <= i32::MAX as u32);
        let nums = nums.iter().map(|&num| num as i32).collect();
        assert_eq!(Solution::check_subarray_sum(nums, k as i32), expected)
    }

    #[test]
    fn ex1() {
        test(&[23, 2, 4, 6, 7], 6, true)
    }

    #[test]
    fn ex2() {
        test(&[23, 2, 6, 4, 7], 6, true)
    }

    #[test]
    fn ex3() {
        test(&[23, 2, 6, 4, 7], 13, false)
    }

    #[test]
    fn discussion_case1() {
        test(&[23, 2, 4, 6, 7], 7, true)
    }

    #[test]
    fn discussion_case2() {
        test(&[5, 0, 0], 3, true)
    }

    #[test]
    fn myex0() {
        for k in 1..=100 {
            test(&[0, 0], k, true)
        }
    }

    #[test]
    fn discussion_case6() {
        test(&[4, 1, 2, 3], 6, true)
    }

    #[test]
    fn discussion_case7() {
        test(&[0], 1, false)
    }

    #[test]
    fn discussion_case8() {
        test(
            &[
                405, 504, 203, 96, 43, 50, 214, 327, 120, 345, 33, 314, 377, 62, 431, 379, 114,
                208, 106, 345, 391, 513, 9, 405, 452, 186, 295, 33, 423, 122, 355, 311, 192, 429,
                320, 360, 85, 96, 32, 258, 407, 71, 436, 370, 365, 199, 443, 521, 262, 232, 355,
                241, 250, 10, 258, 324, 335, 446, 474, 385, 74, 101, 111, 162, 349, 149, 51, 399,
                371, 139, 199, 264, 118, 155, 134, 518, 388, 113, 520, 441, 384, 449, 14, 104, 267,
                92, 477, 50, 505, 368, 466, 519, 105, 443, 31, 21, 485, 146, 115, 94,
            ],
            337,
            true,
        )
    }

    #[test]
    fn discussion_case9() {
        test(
            &[
                422, 224, 184, 178, 189, 290, 196, 236, 281, 464, 351, 193, 49, 76, 0, 298, 193,
                176, 158, 514, 312, 143, 475, 322, 206, 67, 524, 424, 76, 99, 473, 256, 364, 292,
                141, 186, 190, 69, 433, 205, 93, 72, 476, 393, 512, 468, 160, 201, 226, 394, 47,
                140, 389, 51, 142, 135, 349, 244, 16, 356, 440, 188, 16, 133, 58, 394, 7, 517, 56,
                480, 400, 146, 169, 439, 102, 374, 370, 242, 4, 264, 120, 218, 196, 173, 215, 411,
                501, 321, 319, 147, 369, 458, 319, 174, 379, 46, 129, 353, 330, 101,
            ],
            479,
            true,
        )
    }

    #[test]
    fn discussion_case10() {
        test(&[0, 1, 0, 3, 0, 4, 0, 4, 0], 5, false)
    }

    #[test]
    fn failing_case1() {
        test(&[1, 2, 12], 6, false)
    }
}
