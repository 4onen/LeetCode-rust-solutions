// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/

pub struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut longest = 1u8;
        let mut current = 1u8;
        let mut state = std::cmp::Ordering::Equal;
        for i in 1..nums.len() {
            let new_state = nums[i].cmp(&nums[i - 1]);
            match (new_state, state) {
                (std::cmp::Ordering::Equal, _) => {
                    if current > longest {
                        longest = current;
                    }
                    current = 1;
                }
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less)
                | (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                    current += 1;
                }
                _ => {
                    if current > longest {
                        longest = current;
                    }
                    current = 2;
                }
            }
            state = new_state;
        }
        if current > longest {
            longest = current;
        }
        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 50);
        assert!(expected <= nums.len() as i32);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 50);
        }
        assert_eq!(
            Solution::longest_monotonic_subarray(nums.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 4, 3, 3, 2], 2)
    }

    #[test]
    fn ex2() {
        test(&[3, 3, 3, 3], 1)
    }

    #[test]
    fn ex3() {
        test(&[3, 2, 1], 3)
    }

    #[test]
    fn my_extreme_ex1() {
        // All ascending
        let mut input = vec![1; 50];
        for i in 1..50 {
            input[i] = i as i32 + 1;
        }
        test(&input, 50);
    }

    #[test]
    fn my_extreme_ex2() {
        // All descending
        let mut input = vec![50; 50];
        for i in 1..50 {
            input[i] = 50 - i as i32;
        }
        test(&input, 50);
    }

    #[test]
    fn my_extreme_ex3() {
        // Alternating
        let mut input = vec![1; 50];
        for i in 1..50 {
            if i % 2 == 0 {
                input[i] = i as i32 + 1;
            } else {
                input[i] = 50 - i as i32;
            }
        }
        test(&input, 2);
    }

    #[test]
    fn my_extreme_ex4() {
        // Alternating with a single non-alternating element
        let mut input = vec![1; 50];
        for i in 1..50 {
            if i % 2 == 0 {
                input[i] = i as i32 + 1;
            } else {
                input[i] = 50 - i as i32;
            }
        }
        input[1] = 2;
        test(&input, 4);
    }

    #[test]
    fn discussion_case1() {
        test(&[1], 1)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 1], 1)
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 1, 5], 2)
    }

    #[test]
    fn discussion_case4() {
        test(&[1, 3, 4, 5, 7, 4, 6, 8], 5)
    }

    #[test]
    fn discussion_case5() {
        test(&[40, 40, 19, 42, 39, 40, 27, 14, 22, 24, 15], 3)
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                32, 50, 2, 3, 49, 8, 8, 12, 17, 36, 34, 48, 26, 44, 46, 38, 25, 10, 39, 45, 17, 4,
                15, 10, 17, 2, 49, 18, 36, 27, 43, 21, 33,
            ],
            4,
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                49, 4, 40, 45, 20, 48, 18, 50, 20, 44, 30, 45, 40, 20, 26, 27, 30, 33, 31, 27, 41,
                10, 8, 18, 41, 14, 26, 46, 17, 24, 49, 5, 12, 39, 38, 45, 36, 19, 42, 10, 33, 21,
                23, 39, 26,
            ],
            5,
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            &[
                5, 44, 30, 12, 16, 43, 40, 47, 29, 41, 19, 4, 23, 3, 30, 31, 34, 50, 48, 8, 35, 45,
                43, 28, 13, 12, 10, 33, 40, 22, 45, 31, 39, 13, 49, 1, 41, 4, 44, 44, 31, 34, 38,
                47, 45, 18, 32, 7,
            ],
            6,
        )
    }
}
