// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/

pub struct Solution;

impl Solution {
    fn longest_increasing_subseq_lens<T>(nums: T) -> Vec<u16>
    where
        T: std::iter::Iterator<Item = i32> + std::iter::ExactSizeIterator,
    {
        let mut lengths = vec![1; nums.len()];
        let mut len = 1u16;
        let mut nums = nums.enumerate();
        let (_, firstnum) = nums.next().unwrap();
        let mut dp = vec![firstnum; nums.len()];
        for (i, num) in nums {
            match dp[len as usize - 1].cmp(&num) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    dp[len as usize] = num;
                    len += 1;
                }
                std::cmp::Ordering::Greater => match dp[..len as usize].binary_search(&num) {
                    Ok(_) => {}
                    Err(insertion_pos) => dp[insertion_pos] = num,
                },
            }
            lengths[i] = len;
        }
        lengths
    }
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let mut dec = Self::longest_increasing_subseq_lens(nums.iter().copied().rev());
        dec.reverse();
        let inc = Self::longest_increasing_subseq_lens(nums.into_iter());
        len - std::iter::zip(inc.into_iter(), dec.into_iter())
            .filter(|&(inc, dec)| inc > 1 && dec > 1)
            .map(|(inc, dec)| inc as i32 + dec as i32 - 1)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lis(nums: &[i32], expected: &[u16]) {
        assert!(nums.len() >= 3);
        assert!(nums.len() <= 1000);
        for &el in nums {
            assert!(el >= 1);
            assert!(el <= 1_000_000_000);
        }
        assert_eq!(
            Solution::longest_increasing_subseq_lens(nums.into_iter().copied()),
            expected
        );
    }

    #[test]
    fn my_lisex_1() {
        test_lis(&[1, 3, 1], &[1, 2, 2])
    }

    #[test]
    fn my_lisex_2() {
        test_lis(&[2, 1, 1, 5, 6, 2, 3, 1], &[1, 1, 1, 2, 3, 3, 3, 3])
    }

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 3);
        assert!(nums.len() <= 1000);
        for &el in nums {
            assert!(el >= 1);
            assert!(el <= 1_000_000_000);
        }
        assert_eq!(Solution::minimum_mountain_removals(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 3, 1], 0)
    }

    #[test]
    fn ex2() {
        test(&[2, 1, 1, 5, 6, 2, 3, 1], 3)
    }

    #[test]
    fn discussion_case1() {
        test(&[100, 92, 89, 77, 74, 66, 64, 66, 64], 6)
    }

    #[test]
    fn discussion_case2() {
        test(&[9, 8, 1, 7, 6, 5, 4, 3, 2, 1], 2)
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 1, 1, 2, 2, 3, 3, 3, 4, 5, 8, 4, 3, 2, 1], 5)
    }
}
