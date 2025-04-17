// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/

pub struct Solution;

// Naive sol'n (all pairs)
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        assert!(k > 0);
        let k = k as usize;
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && (i * j) % k == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert!(k >= 1);
        assert!(k <= 100);
        assert_eq!(
            Solution::count_pairs(nums.iter().map(|&x| x as i32).collect(), k as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[3, 1, 2, 2, 2, 1, 3], 2, 4)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4], 1, 0)
    }
}
