// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/

pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut maxes = [0; 2];

        for num in nums {
            if num > maxes[0] {
                maxes[1] = maxes[0];
                maxes[0] = num;
            } else if num > maxes[1] {
                maxes[1] = num;
            }
        }

        (maxes[0] - 1) * (maxes[1] - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
