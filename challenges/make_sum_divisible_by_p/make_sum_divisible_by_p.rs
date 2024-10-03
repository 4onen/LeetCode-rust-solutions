// https://leetcode.com/problems/make-sum-divisible-by-p/

pub struct Solution;

impl Solution {
    pub fn min_subarray(mut nums: Vec<i32>, p: i32) -> i32 {
        let mut last = 0;
        for i in 0..nums.len() {
            last = (last + nums[i]) % p;
            nums[i] = last;
        }
        let keys = nums;
        let target = *keys.last().unwrap();
        if target == 0 {
            return 0;
        }
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        let mut result = keys.len() as i32;
        for i in 0..keys.len() as i32 {
            let key = keys[i as usize];
            let target = (key + p - target) % p;
            if let Some(&j) = map.get(&target) {
                result = std::cmp::min(result, i as i32 - j as i32);
            }
            map.insert(key, i);
        }
        if result == keys.len() as i32 {
            -1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], p: i32, expected: i32) {
        assert_eq!(Solution::min_subarray(nums.to_vec(), p), expected);
    }

    #[test]
    fn ex1() {
        test(&[3, 1, 4, 2], 6, 1)
    }

    #[test]
    fn ex2() {
        test(&[6, 3, 5, 2], 9, 2)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3], 3, 0)
    }
}
