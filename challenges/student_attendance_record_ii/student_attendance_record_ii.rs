// https://leetcode.com/problems/student-attendance-record-ii/

pub struct Solution;

impl Solution {
    pub const fn check_record(mut n: i32) -> i32 {
        const MOD: u32 = 1_000_000_007;
        // States:
        // 0: 0A, 0L
        // 1: 0A, 1L
        // 2: 0A, 2L
        // 3: 1A, 0L
        // 4: 1A, 1L
        // 5: 1A, 2L
        // We start one 0A, 0L state machine,
        // then iterate all possible next-state
        // transitions n times.
        // Remember in each transition that our
        // answer must be modulo 1_000_000_007.
        let mut dp: [u32; 6] = [1, 0, 0, 0, 0, 0];
        while n > 0 {
            let mut new_dp = dp;
            new_dp[0] += dp[1] + dp[2];
            new_dp[0] %= MOD;
            new_dp[1] = dp[0];
            new_dp[2] = dp[1];
            new_dp[3] += new_dp[0] + dp[4] + dp[5];
            new_dp[3] %= MOD;
            new_dp[4] = dp[3];
            new_dp[5] = dp[4];
            dp = new_dp;
            n -= 1;
        }
        let mut result = (dp[0] + dp[1] + dp[2]) % MOD;
        result += (dp[3] + dp[4] + dp[5]) % MOD;
        result %= MOD;
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, expected: i32) {
        assert!(n >= 1 && n <= 10_i32.pow(5));
        assert!(expected >= 0 && expected <= 1_000_000_007);
        assert_eq!(Solution::check_record(n), expected);
    }

    #[test]
    fn ex1() {
        test(2, 8)
    }

    #[test]
    fn ex2() {
        test(1, 3)
    }

    #[test]
    fn ex3() {
        test(10101, 183236316)
    }

    #[test]
    fn discussion_case1() {
        test(28, 530803311)
    }

    #[test]
    fn discussion_case2() {
        test(29, 9569297)
    }

    #[test]
    fn discussion_case3() {
        test(99996, 555387871)
    }
}
