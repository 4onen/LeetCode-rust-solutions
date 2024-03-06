// https://leetcode.com/problems/fair-distribution-of-cookies/
// Identical to https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs/
// bar some constraints.

pub struct Solution;

impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let n = cookies.len() as u8;
        assert!(n >= 1);
        assert!(n <= 8);
        assert!(k > 1);
        if k == n as i32 {
            return cookies.into_iter().max().unwrap();
        }
        assert!(k < n as i32);
        // Precompute the time taken for each subset of jobs.
        let cookie_assignment_cost = (0..(1 << n))
            .map(|i| {
                (0..n)
                    .filter(|&j| i & 1 << j != 0)
                    .map(|j| cookies[j as usize])
                    .sum::<i32>()
            })
            .collect::<Vec<_>>();
        // Track the minimum time taken for each subset of jobs
        // for each number of workers.
        let mut curr = vec![0; 1 << n];
        let mut prev = cookie_assignment_cost.clone(); // 1 worker
        for _ in 1..k as u8 {
            for j in 0..1u16 << n {
                let min = (0..1u16 << n)
                    .filter(|&l| j & l == l)
                    .map(|l| {
                        let v = std::cmp::max(
                            prev[j as usize - l as usize],
                            cookie_assignment_cost[l as usize],
                        );
                        unsafe { std::num::NonZeroI32::new_unchecked(v) }
                    })
                    .min()
                    .unwrap_or(unsafe { std::num::NonZeroI32::new_unchecked(i32::MAX) })
                    .get();
                curr[j as usize] = min;
            }
            std::mem::swap(&mut curr, &mut prev);
        }
        prev[(1 << n) - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::distribute_cookies(vec![8, 15, 10, 20, 8], 2), 31);
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::distribute_cookies(vec![6, 1, 3, 2, 2, 4, 1, 2], 3),
            7
        );
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(
            Solution::distribute_cookies(vec![941, 797, 1475, 638, 191, 712], 3),
            1653
        );
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(Solution::distribute_cookies(vec![20, 13, 18], 2), 31,);
    }
}
