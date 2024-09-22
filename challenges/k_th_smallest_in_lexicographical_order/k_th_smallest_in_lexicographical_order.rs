// https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/

pub struct Solution;

impl Solution {
    pub const fn find_kth_number(n: i32, mut k: i32) -> i32 {
        const fn cal_subtree_size(n: i32, mut cur: i32) -> i32 {
            let mut steps = 0;
            let mut next_state = cur + 1;
            while cur <= n {
                steps += if n + 1 < next_state {
                    n + 1
                } else {
                    next_state
                } - cur;
                cur = cur.saturating_mul(10);
                next_state = next_state.saturating_mul(10);
            }
            steps
        }
        let mut cur = 1;
        k -= 1; // 0 is not included in range
        while k > 0 {
            let steps = cal_subtree_size(n, cur);
            if steps <= k {
                cur += 1;
                k -= steps;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u32, k: u32, expected: i32) {
        assert!(k >= 1);
        assert!(n >= k);
        assert!(n <= 1_000_000_000);
        assert_eq!(Solution::find_kth_number(n as i32, k as i32), expected);
    }

    #[test]
    fn ex1() {
        test(13, 2, 10)
    }

    #[test]
    fn ex2() {
        test(1, 1, 1)
    }

    #[test]
    fn discussion_case1() {
        test(100, 10, 17)
    }

    #[test]
    fn my_extreme_ex1() {
        test(1_000_000_000, 10, 1_000_000_000)
    }

    #[test]
    fn my_extreme_ex2() {
        test(1_000_000_000, 1_000_000_000, 999999999)
    }
}
