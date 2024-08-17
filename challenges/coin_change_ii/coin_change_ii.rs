// https://leetcode.com/problems/coin-change-ii/

pub struct Solution;

// Initial sol'n (Would have hit TLE -- see myex2
// impl Solution {
//     pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
//         let mut hash_map = std::collections::HashMap::<(i32, usize), i32>::new();
//         fn backtrack(
//             amount: i32,
//             coins: &[i32],
//             coins_idx: usize,
//             map: &mut std::collections::HashMap<(i32, usize), i32>,
//         ) -> i32 {
//             if coins_idx == coins.len() {
//                 (amount == 0) as i32
//             } else {
//                 match map.get(&(amount, coins_idx)) {
//                     Some(v) => *v,
//                     None => {
//                         let mut i = 0;
//                         let mut total = 0;
//                         while i * coins[coins_idx] <= amount {
//                             total +=
//                                 backtrack(amount - i * coins[coins_idx], coins, coins_idx + 1, map);
//                             i += 1;
//                         }
//                         total
//                     }
//                 }
//             }
//         }
//         backtrack(amount, &coins, 0, &mut hash_map)
//     }
// }

// Raw backtracker
// impl Solution {
//     pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
//         fn backtrack(amount: i32, coins: &[i32], coins_idx: usize) -> i32 {
//             match amount {
//                 i32::MIN..=-1 => 0,
//                 0 => 1,
//                 _ if coins_idx >= coins.len() => 0,
//                 _ => {
//                     backtrack(amount - coins[coins_idx], coins, coins_idx)
//                         + backtrack(amount, coins, coins_idx + 1)
//                 }
//             }
//         }
//         backtrack(amount, &coins, 0)
//     }
// }

// More intelligent DP (That is, actually remembering to use it)
// impl Solution {
//     pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
//         let mut hash_map = std::collections::HashMap::<(i32, usize), i32>::new();
//         fn backtrack(
//             amount: i32,
//             coins: &[i32],
//             coins_idx: usize,
//             map: &mut std::collections::HashMap<(i32, usize), i32>,
//         ) -> i32 {
//             if let Some(&v) = map.get(&(amount, coins_idx)) {
//                 v
//             } else {
//                 match amount {
//                     i32::MIN..=-1 => 0,
//                     0 => 1,
//                     _ if coins_idx >= coins.len() => 0,
//                     _ => {
//                         let res = backtrack(amount - coins[coins_idx], coins, coins_idx, map)
//                             + backtrack(amount, coins, coins_idx + 1, map);
//                         map.insert((amount, coins_idx), res);
//                         res
//                     }
//                 }
//             }
//         }
//         coins.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
//         backtrack(amount, &coins, 0, &mut hash_map)
//     }
// }

// Fixing my failure to realize I can just memo off amount (and poor dtypes)
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        assert!(amount >= 0);
        assert!(amount <= 5000);
        let amount = amount as u16;
        let mut dp = vec![0i32; amount as usize + 1];
        dp[0] = 1;
        for coin in coins {
            let coin = coin as u16;
            for j in coin..=amount {
                dp[j as usize] += dp[(j-coin) as usize];
            }
        }
        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(amount: i32, coins: &[i32], expected: i32) {
        assert_eq!(Solution::change(amount, coins.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(5, &[1, 2, 5], 4)
    }

    #[test]
    fn ex2() {
        test(3, &[2], 0)
    }

    #[test]
    fn ex3() {
        test(10, &[10], 1)
    }

    #[test]
    fn discussion_case1() {
        test(0, &[7], 1)
    }

    #[test]
    fn discussion_case2() {
        test(3, &[2], 0)
    }

    #[test]
    fn myex1() {
        test(5000, &[1, 2], 2501)
    }
    #[test]
    fn myex2() {
        test(5000, &[4, 2, 3], 522084)
    }
}
