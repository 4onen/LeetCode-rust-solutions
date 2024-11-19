// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/

pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        match k {
            i32::MIN..=0 => panic!("k must be positive"),
            1 => nums.into_iter().max().unwrap() as i64,
            k => {
                let k = k as usize;
                let mut max_sum = 0i64;
                let mut sum = 0i64;
                let mut counts = vec![0; 100_001];
                let mut distinct_count = 0;
                for i in 0..k {
                    let num = nums[i] as usize;
                    if counts[num] == 0 {
                        distinct_count += 1;
                    }
                    counts[num] += 1;
                    sum += nums[i] as i64;
                }
                for i in k..nums.len() {
                    if distinct_count == k && sum > max_sum {
                        max_sum = sum;
                    }
                    let num = nums[i] as usize;
                    if counts[num] == 0 {
                        distinct_count += 1;
                    }
                    counts[num] += 1;
                    sum += nums[i] as i64;
                    let num = nums[i - k] as usize;
                    counts[num] -= 1;
                    if counts[num] == 0 {
                        distinct_count -= 1;
                    }
                    sum -= nums[i - k] as i64;
                }
                if distinct_count == k {
                    max_sum = max_sum.max(sum);
                }
                max_sum
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i64) {
        assert!(k >= 1);
        assert!(nums.len() >= k as usize);
        assert!(nums.len() <= 100_000);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100_000);
        }
        assert_eq!(Solution::maximum_subarray_sum(nums.to_vec(), k), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 5, 4, 2, 9, 9, 9], 3, 15)
    }

    #[test]
    fn ex2() {
        test(&[4, 4, 4], 3, 0)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 1, 1, 7, 8, 9], 3, 7 + 8 + 9)
    }

    #[test]
    fn discussion_case2() {
        test(&[5, 5, 3, 3, 1], 3, 0)
    }

    #[test]
    fn discussion_case3() {
        test(&[100, 1, 2, 100], 2, 2 + 100)
    }

    #[test]
    fn discussion_case4() {
        test(&[9, 9, 9, 1, 2, 3], 3, 9 + 1 + 2)
    }

    #[test]
    fn discussion_case5() {
        test(&[1, 3, 3, 3, 3, 2, 1, 3, 1, 1, 2], 3, 3 + 2 + 1)
    }

    #[test]
    fn discussion_case6() {
        test(&[3, 3, 5, 5, 3, 3, 1, 1, 4, 2, 5, 1], 4, 4 + 2 + 5 + 1)
    }

    #[test]
    fn my_extreme_ex1() {
        let mut input = vec![1; 100_000];
        for i in 0..100_000 {
            input[i] = i as i32 + 1;
        }
        test(&input, 1, 100_000)
    }

    #[test]
    fn my_extreme_ex2() {
        let mut input = vec![1; 100_000];
        for i in 0..100_000 {
            input[i] = i as i32 + 1;
        }
        test(&input, 2, 199_999)
    }
}
