// https://leetcode.com/problems/stone-game-ii/

pub struct Solution;

// DP sol'n
// impl Solution {
//     pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
//         assert!(piles.len() >= 1);
//         assert!(piles.len() <= 100);
//         let n = piles.len() as u8;
//         let mut dp = vec![vec![0; n as usize + 1]; n as usize + 1];
//         let mut suffix_sum = piles;
//         for i in (0..n - 1).rev() {
//             suffix_sum[i as usize] += suffix_sum[i as usize + 1];
//         }
//         for i in (0..n).rev() {
//             for m in 1..=n {
//                 for x in 1..=2 * m {
//                     if i + x > n {
//                         break;
//                     }
//                     let next_m = std::cmp::max(m, x);
//                     dp[i as usize][m as usize] = std::cmp::max(
//                         dp[i as usize][m as usize],
//                         suffix_sum[i as usize] - dp[(i + x) as usize][next_m as usize],
//                     );
//                 }
//             }
//         }
//         dp[0][1]
//     }
// }

// Optimized sol'n (saving a pass)
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        assert!(piles.len() >= 1);
        assert!(piles.len() <= 100);
        let n = piles.len() as u16;
        let mut dp = vec![vec![0; n as usize + 1]; n as usize + 1];
        let mut stones_above = 0;
        for i in (0..n).rev() {
            stones_above += piles[i as usize];
            for m in 1..=n {
                for x in 1..=2 * m {
                    if i + x > n {
                        break;
                    }
                    let next_m = std::cmp::max(m, x);
                    dp[i as usize][m as usize] = std::cmp::max(
                        dp[i as usize][m as usize],
                        stones_above - dp[(i + x) as usize][next_m as usize],
                    );
                }
            }
        }
        dp[0][1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[u16], expected: u32) {
        assert!(input.len() >= 1);
        assert!(input.len() <= 100);
        for &i in input {
            assert!(i >= 1);
            assert!(i <= 10_000);
        }
        let piles = input.iter().map(|&x| x as i32).collect();
        assert_eq!(Solution::stone_game_ii(piles), expected as i32);
    }

    #[test]
    fn ex1() {
        test(&[2, 7, 9, 4, 4], 10)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4, 5, 100], 104)
    }

    #[test]
    fn discussion_case1() {
        test(&[8, 6, 9, 1, 7, 9], 25)
    }

    #[test]
    fn discussion_case2() {
        test(&[86, 11, 7, 6, 46, 37, 72, 67, 33, 25, 54, 45], 273)
    }

    #[test]
    fn discussion_case3() {
        test(&[1], 1)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
                66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
                87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
            ],
            2526,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1,
                100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1,
                100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1,
                100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1,
                100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100,
            ],
            2525,
        )
    }
}
