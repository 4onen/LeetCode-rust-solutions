// https://leetcode.com/problems/grumpy-bookstore-owner/description/

pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut ungrumpy_satisfied = 0;
        let mut grumpy_satisfied = 0;
        let mut max_grumpy_satisfied = 0;
        for i in 0..customers.len() {
            match grumpy[i] {
                0 => ungrumpy_satisfied += customers[i],
                1 => grumpy_satisfied += customers[i],
                _ => unreachable!(),
            }
            if i as i32 >= minutes {
                match grumpy[i - minutes as usize] {
                    0 => {}
                    1 => grumpy_satisfied -= customers[i - minutes as usize],
                    _ => unreachable!(),
                }
            }
            max_grumpy_satisfied = std::cmp::max(max_grumpy_satisfied, grumpy_satisfied);
        }
        ungrumpy_satisfied + max_grumpy_satisfied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(customers: &[i32], grumpy: &[i32], minutes: i32, expected: i32) {
        assert_eq!(
            Solution::max_satisfied(customers.to_vec(), grumpy.to_vec(), minutes),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 0, 1, 2, 1, 1, 7, 5], &[0, 1, 0, 1, 0, 1, 0, 1], 3, 16);
    }

    #[test]
    fn ex2() {
        test(&[1], &[0], 1, 1);
    }

    #[test]
    fn discussion_case1() {
        test(&[10, 1, 7], &[0, 0, 0], 2, 18);
    }

    #[test]
    fn discussion_case2() {
        test(&[4, 10, 10], &[1, 1, 0], 2, 24);
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 2, 3, 4, 5], &[1, 1, 1, 0, 0], 3, 15);
    }

    #[test]
    fn discussion_case4() {
        test(&[10, 1, 10, 1, 10, 1, 10], &[1; 7], 1, 10);
    }
}
