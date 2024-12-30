// https://leetcode.com/problems/count-ways-to-build-good-strings/

pub struct Solution;

// Naive recursive sol'n (stack overflow)
// impl Solution {
//     pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
//         fn rec(low: i32, high: i32, zero: i32, one: i32) -> i32 {
//             if high < 0 {
//                 return 0;
//             }
//             let here = (low <= 0) as i32;
//             let there0 = rec(low - zero, high - zero, zero, one);
//             let there1 = rec(low - one, high - one, zero, one);
//             here + there0 + there1
//         }
//         rec(low, high, zero, one)
//     }
// }

// Memo recursive sol'n
// impl Solution {
//     pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
//         type DPCount = i32;
//         const MODULO: DPCount = 1_000_000_007;
//         let low = low as u32;
//         let high = high as u32;
//         let zero = zero as u32;
//         let one = one as u32;
//         let mut stack: Vec<u32> = vec![0];
//         let mut memo: Vec<DPCount> = vec![-1; (high + 1) as usize];
//         while stack.len() > 0 {
//             let tgt = *stack.last().unwrap();
//             let there0 = {
//                 let memod = memo.get((tgt + zero) as usize).copied().unwrap_or(0);
//                 if memod >= 0 {
//                     memod
//                 } else {
//                     stack.push(tgt + zero);
//                     continue;
//                 }
//             };
//             let there1 = {
//                 let memod = memo.get((tgt + one) as usize).copied().unwrap_or(0);
//                 if memod >= 0 {
//                     memod
//                 } else {
//                     stack.push(tgt + one);
//                     continue;
//                 }
//             };
//             let here = (tgt >= low && tgt <= high) as DPCount;
//             memo[tgt as usize] = (here + there0 + there1) % MODULO;
//             _ = stack.pop();
//         }
//         memo[0]
//     }
// }

// Iterative dp sol'n
impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        type DPCount = i32;
        const MODULO: DPCount = 1_000_000_007;
        let low = low as u32;
        let zero = zero as u32;
        let one = one as u32;
        let mut dp: Vec<DPCount> = vec![0; (high + 1) as usize];
        for i in (0..dp.len() as u32).rev() {
            let here = (i >= low) as DPCount;
            let there0 = dp.get((i + zero) as usize).copied().unwrap_or(0);
            let there1 = dp.get((i + one) as usize).copied().unwrap_or(0);
            dp[i as usize] = (here + there0 + there1) % MODULO;
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(low: u32, high: u32, zero: u32, one: u32, expected: i32) {
        assert!(low >= 1);
        assert!(low <= high);
        assert!(high <= 100_000);
        assert!(zero >= 1);
        assert!(one >= 1);
        assert!(zero <= low);
        assert!(one <= low);
        assert_eq!(
            Solution::count_good_strings(low as i32, high as i32, zero as i32, one as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(3, 3, 1, 1, 8)
    }

    #[test]
    fn ex2() {
        test(2, 3, 1, 2, 5)
    }

    #[test]
    fn discussion_case1() {
        test(45360, 45360, 45360, 2, 2)
    }
}
