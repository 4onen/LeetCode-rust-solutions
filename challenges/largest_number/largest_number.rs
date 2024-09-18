// https://leetcode.com/problems/largest-number/

pub struct Solution;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_unstable_by(|a, b| {
            if a == b {
                return std::cmp::Ordering::Equal;
            }
            if *a<=0 || *b<=0 {
                return a.cmp(&b).reverse();
            }
            let ab = *a as u64*10_u64.pow(b.ilog10() as u32+1) + *b as u64;
            let ba = *b as u64*10_u64.pow(a.ilog10() as u32+1) + *a as u64;
            ba.cmp(&ab)
        });
        if nums[0] == 0 {
            return "0".to_string();
        }
        nums.iter().map(|n| n.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: &str) {
        assert_eq!(Solution::largest_number(nums.to_vec()), expected)
    }

    #[test]
    fn ex1() {
        test(&[10, 2], "210")
    }

    #[test]
    fn ex2() {
        test(&[3, 30, 34, 5, 9], "9534330")
    }

    #[test]
    fn myex1() {
        test(&[1], "1")
    }

    #[test]
    fn discussion_case1() {
        test(&[0, 0, 0], "0")
    }

    #[test]
    fn myex0() {
        test(&[1,0], "10")
    }

    #[test]
    fn discussion_case2() {
        test(&[3,30,34,5,9], "9534330")
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[999_999_999, 999_999_998], "999999999999999998")
    }
}
