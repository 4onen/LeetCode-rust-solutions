// https://leetcode.com/problems/climbing-stairs/

pub struct Solution;

// DP Sol'n
// impl Solution {
//     pub fn climb_stairs(n: i32) -> i32 {
//         let mut dp = vec![0; n as usize + 1];
//         dp[0] = 1;
//         dp[1] = 1;
//         for i in 2..=n as usize {
//             dp[i] = dp[i - 1] + dp[i - 2];
//         }
//         dp[n as usize]
//     }
// }

// Constant Space Sol'n
// impl Solution {
//     pub fn climb_stairs(n: i32) -> i32 {
//         let mut one_back = 1;
//         let mut two_back = 1;
//         for _ in 2..=n {
//             let tmp = one_back + two_back;
//             two_back = one_back;
//             one_back = tmp;
//         }
//         one_back
//     }
// }

// Const constant space sol'n
impl Solution {
    pub const fn climb_stairs(n: i32) -> i32 {
        let mut one_back = 1;
        let mut two_back = 1;
        let mut i = 2;
        while i <= n {
            let tmp = one_back + two_back;
            two_back = one_back;
            one_back = tmp;
            i += 1;
        }
        one_back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}
