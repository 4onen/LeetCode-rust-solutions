// https://leetcode.com/problems/distinct-subsequences/

pub struct Solution;

// 2D DP sol'n
// impl Solution {
//     pub fn num_distinct(s: String, t: String) -> i32 {
//         let s = s.as_bytes();
//         let t = t.as_bytes();
//         let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
//         for i in 0..=s.len() {
//             dp[i][0] = 1;
//         }
//         for i in 1..=s.len() {
//             for j in 1..=t.len() {
//                 if s[i - 1] == t[j - 1] {
//                     dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
//                 } else {
//                     dp[i][j] = dp[i - 1][j];
//                 }
//             }
//         }
//         dp[s.len()][t.len()]
//     }
// }

// 1D DP sol'n (one alloc!)
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        if s.len() == t.len() {
            return if s == t { 1 } else { 0 };
        }
        let mut dp: Vec<u32> = vec![0; 2 * (t.len() + 1)];
        let (mut curr, mut prev) = dp.split_at_mut(t.len() + 1);
        prev[0] = 1;
        for i in 1..=s.len() {
            curr[0] = 1;
            for j in 1..=t.len() {
                if s[i - 1] == t[j - 1] {
                    curr[j] = u32::saturating_add(prev[j - 1], prev[j]);
                } else {
                    curr[j] = prev[j];
                }
            }
            std::mem::swap(&mut curr, &mut prev);
        }
        if prev[t.len()] > i32::MAX as u32 {
            -1
        } else {
            prev[t.len()] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let s = "rabbbit";
        let t = "rabbit";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 3);
    }

    #[test]
    fn ex2() {
        let s = "babgbag";
        let t = "bag";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 5);
    }

    #[test]
    fn myex1() {
        let s = "abc";
        let t = "abc";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn myex2() {
        let s = "abc";
        let t = "a";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn myex3() {
        let s = "abc";
        let t = "b";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn myex4() {
        let s = "abc";
        let t = "c";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn myex5() {
        let s = "abc";
        let t = "ab";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn myex6() {
        let s = "abc";
        let t = "bc";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn myex7() {
        let s = "abc";
        let t = "ac";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn myex8() {
        let s = "abcabc";
        let t = "a";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 2);
    }

    #[test]
    fn myex9() {
        let s = "abcabc";
        let t = "ab";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 3);
    }

    #[test]
    fn myex10() {
        let s = "abcabc";
        let t = "abc";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 4);
    }

    #[test]
    fn myex11() {
        let s = "abcabc";
        let t = "abca";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1);
    }

    #[test]
    fn my_extreme_ex1() {
        let s = std::iter::repeat('a').take(1000).collect::<String>();
        let t = "a";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 1000);
    }

    #[test]
    fn my_extreme_ex2() {
        let s = std::iter::repeat('a').take(1000).collect::<String>();
        let t = "aa";
        assert_eq!(
            Solution::num_distinct(s.to_owned(), t.to_owned()),
            1000 * 999 / 2
        );
    }

    #[test]
    fn discussion_case1() {
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let t = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(Solution::num_distinct(s.to_owned(), t.to_owned()), 0)
    }
}
