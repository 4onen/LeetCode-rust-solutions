// https://leetcode.com/problems/three-consecutive-odds/

pub struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for i in arr {
            if i & 1 == 1 {
                count += 1;
                if count == 3 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], expected: bool) {
        assert_eq!(Solution::three_consecutive_odds(arr.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 6, 4, 1], false);
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 34, 3, 4, 5, 7, 23, 12], true);
    }

    #[test]
    fn myex1() {
        test(&[1, 2, 3, 4, 5, 6, 7, 8, 9], false);
    }

    #[test]
    fn myex2() {
        test(&[1, 3, 5, 7, 9, 11, 13, 15, 17], true);
    }
}
