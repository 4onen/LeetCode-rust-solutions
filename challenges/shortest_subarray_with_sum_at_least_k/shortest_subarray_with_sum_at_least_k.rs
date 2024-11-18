// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/description/

pub struct Solution;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = u32::MAX;
        let mut cur_sum: i64 = 0;
        let mut q: std::collections::VecDeque<(i64, u32)> = std::collections::VecDeque::new();
        for i in 0..nums.len() as u32 {
            cur_sum += nums[i as usize] as i64;
            if cur_sum >= k as i64 {
                res = std::cmp::min(res, i + 1);
            }
            while (!q.is_empty()) && cur_sum - q[0].0 >= k as i64 {
                let (_, end_idx) = q.pop_front().unwrap();
                res = std::cmp::min(res, (i - end_idx) as u32);
            }
            while (!q.is_empty()) && q[q.len() - 1].0 > cur_sum {
                q.pop_back();
            }
            q.push_back((cur_sum, i))
        }
        return if res == u32::MAX { -1 } else { res as i32 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        assert!(k >= 0);
        assert!(k <= 1_000_000_000);
        for &num in nums {
            assert!(num >= -100_000);
            assert!(num <= 100_000);
        }
        assert_eq!(Solution::shortest_subarray(nums.to_vec(), k), expected);
    }

    #[test]
    fn ex1() {
        test(&[1], 1, 1)
    }

    #[test]
    fn ex2() {
        test(&[1, 2], 4, -1)
    }

    #[test]
    fn ex3() {
        test(&[2, -1, 2], 3, 3)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                -34, 37, 51, 3, -12, -50, 51, 100, -47, 99, 34, 14, -13, 89, 31, -14, -44, 23, -38,
                6,
            ],
            151,
            2,
        )
    }

    #[test]
    fn failing_case1() {
        test(&[-100000; 100_000], 1000000000, -1)
    }
}
