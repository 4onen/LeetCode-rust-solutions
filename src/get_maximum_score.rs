// https://leetcode.com/problems/get-the-maximum-score/

pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut iter1 = nums1.into_iter().peekable();
        let mut iter2 = nums2.into_iter().peekable();
        let mut confirmed_sum = 0;
        let mut possible_sum1: i64 = 0;
        let mut possible_sum2: i64 = 0;

        while iter1.peek().is_some() || iter2.peek().is_some() {
            match (iter1.peek(), iter2.peek()) {
                (Some(&v1), Some(&v2)) if v1 == v2 => {
                    confirmed_sum = (confirmed_sum as i64
                        + std::cmp::max(possible_sum1, possible_sum2)
                        + v1 as i64)
                        % MOD as i64;
                    possible_sum1 = 0;
                    possible_sum2 = 0;
                    iter1.next();
                    iter2.next();
                }
                (Some(&v1), Some(&v2)) if v1 < v2 => {
                    possible_sum1 = possible_sum1 + v1 as i64;
                    iter1.next();
                }
                (Some(&v1), Some(&v2)) if v1 > v2 => {
                    possible_sum2 = possible_sum2 + v2 as i64;
                    iter2.next();
                }
                (Some(&v1), None) => {
                    possible_sum1 = possible_sum1 + v1 as i64;
                    iter1.next();
                }
                (None, Some(&v2)) => {
                    possible_sum2 = possible_sum2 + v2 as i64;
                    iter2.next();
                }
                _ => unreachable!(),
            }
        }

        ((confirmed_sum + std::cmp::max(possible_sum1, possible_sum2)) % MOD as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::max_sum(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]),
            30
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::max_sum(vec![1, 3, 5, 7, 9], vec![3, 5, 100]), 109);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::max_sum(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]),
            40
        );
    }

    #[test]
    fn myex1() {
        let input = (0..100_000).into_iter().collect::<Vec<_>>();
        assert_eq!(
            Solution::max_sum(input.clone(), input.clone(),),
            input.into_iter().fold(0, |acc, v| (acc + v) % MOD)
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::max_sum(vec![1, 2, 3], vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn myex3() {
        let input = (0..100_000)
            .into_iter()
            .map(|v| 10_000_000 - v)
            .collect::<Vec<_>>();
        assert_eq!(
            Solution::max_sum(input.clone(), input.clone(),),
            input.into_iter().fold(0, |acc, v| (acc + v) % MOD)
        );
    }

    #[test]
    fn myex4() {
        let input = (0..100_000)
            .into_iter()
            .map(|v| 10_000_000 - v)
            .collect::<Vec<_>>();
        assert_eq!(
            Solution::max_sum(input.clone(), vec![]),
            input.into_iter().fold(0, |acc, v| (acc + v) % MOD)
        );
    }
}
