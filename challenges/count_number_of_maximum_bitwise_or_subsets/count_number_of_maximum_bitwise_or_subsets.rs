// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/

pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        const fn subsets(nums: &[i32], target: i32, state: i32, index: u8) -> i32 {
            if index as usize >= nums.len() {
                return (target == state) as i32;
            }
            if state == target {
                return 2_i32.pow(nums.len() as u32 - index as u32);
            }
            let new_state = state | nums[index as usize];
            let subsets_beyond_without = subsets(nums, target, state, index + 1);
            if new_state == state {
                2 * subsets_beyond_without
            } else {
                subsets(nums, target, new_state, index + 1) + subsets_beyond_without
            }
        }
        let max_or = nums.iter().copied().reduce(|acc, a| acc | a).unwrap();
        subsets(&nums, max_or, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 16);
        let mut i = 0;
        while i < nums.len() {
            let num = nums[i];
            assert!(num >= 1);
            assert!(num <= 100_000);
            i += 1;
        }
        assert_eq!(Solution::count_max_or_subsets(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[3, 1], 2)
    }

    #[test]
    fn ex2() {
        test(&[2, 2, 2], 7)
    }

    #[test]
    fn ex3() {
        test(&[3, 2, 1, 5], 6)
    }

    #[test]
    fn discussion_case1() {
        test(&[12007], 1)
    }

    #[test]
    fn discussion_case2() {
        test(&[39317, 28735, 71230, 59313, 19954], 5)
    }

    #[test]
    fn discussion_case3() {
        test(&[35569, 91997, 54930, 66672, 12363], 6)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                32, 39, 37, 31, 42, 38, 29, 43, 40, 36, 44, 35, 41, 33, 34, 30,
            ],
            57083,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                63609, 94085, 69432, 15248, 22060, 76843, 84075, 835, 23463, 66399, 95031, 22676,
                92115,
            ],
            3772,
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                6890, 6890, 6890, 6890, 6890, 6890, 6890, 6890, 6890, 6890, 6890, 6890, 6890, 6890,
                6890, 6890,
            ],
            65535,
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                90193, 56697, 77229, 72927, 74728, 93652, 70751, 32415, 94774, 9067, 14758, 59835,
                26047, 36393, 60530, 64649,
            ],
            60072,
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            &[
                89260, 58129, 16949, 64128, 9782, 26664, 96968, 59838, 27627, 68561, 4026, 91345,
                26966, 28876, 46276, 19878,
            ],
            52940,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let input = [1; 16];
        test(&input, 2_i32.pow(16) - 1)
    }

    #[test]
    fn my_extreme_ex2() {
        let mut input = [1; 16];
        input[15] = 2;
        test(&input, 2_i32.pow(15) - 1)
    }
}
